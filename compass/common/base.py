"""Common ordinance extraction components"""

import asyncio
import logging
from datetime import datetime

import networkx as nx
from elm import ApiBase

from compass.common.tree import AsyncDecisionTree
from compass.utilities import llm_response_as_json
from compass.utilities.enums import LLMUsageCategory
from compass.utilities.parsing import (
    merge_overlapping_texts,
    clean_backticks_from_llm_response,
)
from compass.exceptions import COMPASSRuntimeError


logger = logging.getLogger(__name__)
_SECTION_PROMPT = (
    "The value of the 'section' key should be a string representing the "
    "title of the section (including numerical labels), if it's given, "
    "and `null` otherwise."
)
_SUMMARY_PROMPT = (
    "The value of the 'summary' key should be a short summary "
    "of the ordinance, using direct text excerpts as much as possible."
)
_UNITS_IN_SUMMARY_PROMPT = (
    "Include any clarifications about the units in the summary."
)
EXTRACT_ORIGINAL_TEXT_PROMPT = (
    "Extract all portions of the text (with original formatting) "
    "that state how close I can site {tech} to {feature}. "
    "{feature_clarifications}"
    "Focus on ordinances relating to setbacks from {feature}; do not "
    "respond based on any text related to {ignore_features}. "
    "The extracted text will be used for structured data extraction, so it "
    "must be both **comprehensive** (retaining all relevant details) and "
    "**focused** (excluding unrelated content). Ensure that all retained "
    "information is **directly applicable** to systems that would typically "
    "be defined as {tech} based on the text itself — for example, systems "
    "intended for electricity generation or sale, or those above "
    "thresholds such as height or rated capacity. Ignore any text that "
    "applies **only** to smaller or clearly non-commercial systems. "
)


def setup_graph_no_nodes(**kwargs):
    """Setup a graph with no nodes

    This function is used to set keywords on the graph that can be used
    in text prompts on the graph nodes.

    Returns
    -------
    nx.DiGraph
        Graph with no nodes but with global keywords set.
    """
    return nx.DiGraph(
        SECTION_PROMPT=_SECTION_PROMPT,
        SUMMARY_PROMPT=_SUMMARY_PROMPT,
        UNITS_IN_SUMMARY_PROMPT=_UNITS_IN_SUMMARY_PROMPT,
        **kwargs,
    )


def llm_response_starts_with_yes(response):
    """Check if LLM response begins with "yes" (case-insensitive)

    Parameters
    ----------
    response : str
        LLM response string.

    Returns
    -------
    bool
        `True` if LLM response begins with "Yes".
    """
    return response.lower().startswith("yes")


def llm_response_starts_with_no(response):
    """Check if LLM response begins with "no" (case-insensitive)

    Parameters
    ----------
    response : str
        LLM response string.

    Returns
    -------
    bool
        `True` if LLM response begins with "No".
    """
    return response.lower().startswith("no")


def llm_response_does_not_start_with_no(response):
    """Check if LLM response does not start with "no" (case-insensitive)

    Parameters
    ----------
    response : str
        LLM response string.

    Returns
    -------
    bool
        `True` if LLM response does not begin with "No".
    """
    return not llm_response_starts_with_no(response)


def setup_async_decision_tree(
    graph_setup_func, usage_sub_label=None, **kwargs
):
    """Setup Async Decision tree for ordinance extraction"""
    G = graph_setup_func(**kwargs)  # noqa: N806
    tree = AsyncDecisionTree(G, usage_sub_label=usage_sub_label)
    assert len(tree.chat_llm_caller.messages) == 1
    return tree


def found_ord(messages):
    """Check messages from the base graph to see if ordinance was found

    ..IMPORTANT:: This function may break if the base graph structure
                  changes. Always update the hardcoded values to match
                  the base graph message containing the LLM response
                  about ordinance content.
    """
    num_messages_in_base_tree = 3
    if len(messages) < num_messages_in_base_tree:
        return False
    return llm_response_starts_with_yes(messages[2].get("content", ""))


async def run_async_tree(tree, response_as_json=True):
    """Run Async Decision Tree and return output as dict"""
    try:
        response = await tree.async_run()
    except COMPASSRuntimeError:
        response = None

    if response_as_json:
        return llm_response_as_json(response) if response else {}

    return response


async def run_async_tree_with_bm(tree, base_messages):
    """Run Async Decision Tree from base messages; return dict output"""
    tree.chat_llm_caller.messages = base_messages
    assert len(tree.chat_llm_caller.messages) == len(base_messages)
    return await run_async_tree(tree)


def empty_output(feature):
    """Empty output for a feature (not found in text)"""
    if feature in {"structures", "property line"}:
        return [
            {"feature": f"{feature} (participating)"},
            {"feature": f"{feature} (non-participating)"},
        ]
    return [{"feature": feature}]


def setup_base_setback_graph(**kwargs):
    """Setup graph to get setback ordinance text for a given feature

    Parameters
    ----------
    **kwargs
        Keyword-value pairs to add to graph.

    Returns
    -------
    nx.DiGraph
        Graph instance that can be used to initialize an
        `elm.tree.DecisionTree`.
    """
    G = setup_graph_no_nodes(**kwargs)  # noqa: N806

    G.add_node(
        "init",
        prompt=(
            "Is there text in the following legal document that describes "
            "how far I have to setback {tech} from {feature}? "
            "{feature_clarifications}"  # expected to end in space
            "Focus only on setbacks from {feature}; do not respond "
            "based on any text related to {ignore_features}. "
            "Please only consider setbacks specifically for systems that "
            "would typically be defined as {tech} based on the text itself "
            "— for example, systems intended for electricity generation or "
            "sale, or those above thresholds such as height or rated "
            "capacity. Ignore any requirements that apply only to smaller "
            "or clearly non-commercial systems. "
            "Don't forget to pay extra attention to clarifying text found "
            "in parentheses and footnotes. "
            "Please start your response with either 'Yes' or 'No' and briefly "
            "explain your answer."
            '\n\n"""\n{text}\n"""'
        ),
    )

    G.add_edge(
        "init", "get_text", condition=llm_response_does_not_start_with_no
    )
    G.add_node("get_text", prompt=EXTRACT_ORIGINAL_TEXT_PROMPT)

    return G


def setup_participating_owner(**kwargs):
    """Setup graph to check for "participating" setbacks for a feature

    Parameters
    ----------
    **kwargs
        Keyword-value pairs to add to graph.

    Returns
    -------
    nx.DiGraph
        Graph instance that can be used to initialize an
        `elm.tree.DecisionTree`.
    """
    G = setup_graph_no_nodes(**kwargs)  # noqa: N806

    G.add_node(
        "init",
        prompt=(
            "Does the ordinance for {feature} setbacks explicitly specify "
            "a value that applies to **participating** {owned_type} owners? "
            "{feature_clarifications}"
            "Focus only on setbacks from {feature}; do not respond "
            "based on any text related to {ignore_features}. "
            "Please only consider setbacks specifically for systems that "
            "would typically be defined as {tech} based on the text itself "
            "— for example, systems intended for electricity generation or "
            "sale, or those above thresholds such as height or rated "
            "capacity. Ignore any requirements that apply only to smaller "
            "or clearly non-commercial systems. "
            "If your answer is 'yes', justify it by quoting the raw text "
            "directly."
        ),
    )
    G.add_edge("init", "non_part")
    G.add_node(
        "non_part",
        prompt=(
            "Does the ordinance for {feature} setbacks explicitly specify "
            "a value that applies to **non-participating** {owned_type} "
            "owners? {feature_clarifications}"
            "Focus only on setbacks from {feature}; do not respond "
            "based on any text related to {ignore_features}. "
            "Please only consider setbacks specifically for systems that "
            "would typically be defined as {tech} based on the text itself "
            "— for example, systems intended for electricity generation or "
            "sale, or those above thresholds such as height or rated "
            "capacity. Ignore any requirements that apply only to smaller "
            "or clearly non-commercial systems. "
            "If your answer is 'yes', justify it by quoting the raw text "
            "directly."
        ),
    )
    G.add_edge("non_part", "final")
    G.add_node(
        "final",
        prompt=(
            "Please respond based on our entire conversation so far. "
            "Return your answer as a single dictionary in JSON format (not "
            "markdown). Your JSON file must include exactly two keys. The "
            "keys are 'participating' and 'non-participating'. The value of "
            "the 'participating' key should be a string containing the raw "
            "text with original formatting from the ordinance that applies to "
            "**participating** owners if you answered 'yes' to the first "
            "question or `null` if you answered 'no'. The value of the "
            "'non-participating' key should be a string containing the raw "
            "text with original formatting from the ordinance that applies to "
            "**non-participating** owners _or_ simply the full ordinance for "
            "{feature} setbacks if the text did not make the distinction "
            "between **participating** and **non-participating** owners."
        ),
    )
    return G


def setup_graph_extra_restriction(is_numerical=True, **kwargs):
    """Setup Graph to extract non-setback ordinance values from text

    Parameters
    ----------
    **kwargs
        Keyword-value pairs to add to graph.

    Returns
    -------
    nx.DiGraph
        Graph instance that can be used to initialize an
        `elm.tree.DecisionTree`.
    """
    kwargs.setdefault("unit_clarification", "")
    kwargs.setdefault("feature_clarifications", "")
    G = setup_graph_no_nodes(**kwargs)  # noqa: N806

    G.add_node(
        "init",
        prompt=(
            "Does the following legal text explicitly enact {restriction} for "
            "{tech} for a particular jurisdiction that an energy system "
            "developer would have to abide to? {feature_clarifications}\n"
            "Make sure your answer adheres to these guidelines:\n"
            "1) Respond based only on the explicit text provided for "
            "{restriction}. Do not infer or assume relevance based on general "
            "definitions, interpretations, or overlap with other categories. "
            "Do not include content just because it could be considered "
            "related to {restriction} by definition. If the text does not "
            "directly mention or clearly describe {restriction} for {tech}, "
            "respond with 'No'.\n"
            "2) If the text only provides a definition of what {restriction} "
            "are without providing specifics, please respond with 'No'.\n"
            "3) Focus only on {restriction} specifically for systems that "
            "would typically be defined as {tech} based on the text itself. "
            "Ignore any requirements that apply only to smaller or clearly "
            "non-commercial systems. "
            "4) Pay close attention to clarifying details in parentheses, "
            "footnotes, or additional explanatory text.\n"
            "5) Please start your response with either 'Yes' or 'No' and "
            "briefly explain your answer."
            '\n\n"""\n{text}\n"""'
        ),
    )

    if is_numerical:
        if "other" in kwargs.get("restriction", ""):
            G.add_edge(
                "init",
                "is_intra_farm",
                condition=llm_response_starts_with_yes,
            )
            G.add_node(
                "is_intra_farm",
                prompt=(
                    "Does the separation requirement apply to full farms "
                    "and/or utility-size installations? If so, please start "
                    "your answer with 'Yes'. If the separation requirement "
                    "only applies to individual farm components (i.e. "
                    "individual energy generation system units), please start "
                    "your response with 'No'. In either case, briefly explain "
                    "your answer."
                ),
            )
            G.add_edge(
                "is_intra_farm",
                "value",
                condition=llm_response_starts_with_yes,
            )
        else:
            G.add_edge("init", "value", condition=llm_response_starts_with_yes)

        G.add_node(
            "value",
            prompt=(
                "What is the **numerical** value given for the {restriction} "
                "for {tech}? Follow these guidelines:\n"
                "1) Extract only the explicit numerical value provided for "
                "the restriction. Do not infer values from related "
                "restrictions.\n"
                "2) If multiple values are given, select the most restrictive "
                "one (i.e., the smallest allowable limit, the lowest maximum, "
                "etc.).\n"
                "3) Focus only on {restriction} specifically for systems that "
                "would typically be defined as {tech} based on the text "
                "itself. Ignore any requirements that apply only to smaller "
                "or clearly non-commercial systems. "
                "4) Pay close attention to clarifying details in parentheses, "
                "footnotes, or additional explanatory text.\n\n"
                "Example Inputs and Outputs:\n"
                "Text: 'For all WES there is a limitation of overall height "
                "of 200 feet (including blades).'\n"
                "Output: 200\n"
                "Text: 'The noise level of all SES shall be no greater than "
                "thirty-two (32) decibels measured from the nearest property "
                "line. This level may only be exceeded during short-term "
                "events such as utility outages and/or severe wind storms.'\n"
                "Output: 32\n"
                "Text: 'At no time shall a wind turbine tower, nacelle, or "
                "blade create shadow flicker on any non-participating "
                "landowner property'\n"
                "Output: 0\n"
                "Text: Solar Panels shall not exceed 22'6\" in height. The "
                "height is determined from the ground to the top of the panel "
                "at any angle.\n"
                "Output: 22.5\n"
            ),
        )

        G.add_edge("value", "units")
        G.add_node(
            "units",
            prompt=(
                "What are the units for the {restriction} for {tech}? Ensure "
                "that:\n"
                "1) You accurately identify the unit value associated with "
                "the restriction.\n"
                "2) The unit is expressed using standard, conventional unit "
                "names (e.g., 'feet', 'meters', 'acres', 'dBA', etc.). "
                "{unit_clarification}\n"
                "3) If multiple values are mentioned, return only the units "
                "for the most restrictive value that directly pertains to the "
                "restriction.\n"
                "\nExample Inputs and Outputs:\n"
                "Text: 'For all WES there is a limitation of overall height "
                "of 200 feet (including blades).'\n"
                "Output: 'feet'\n"
                "Text: 'The noise level of all SES shall be no greater than "
                "thirty-two (32) decibels measured from the nearest property "
                "line. This level may only be exceeded during short-term "
                "events such as utility outages and/or severe wind storms.'\n"
                "Output: 'dBA'\n"
                "Text: 'At no time shall a wind turbine tower, nacelle, or "
                "blade create shadow flicker on any non-participating "
                "landowner property'\n"
                "Output: 'hr/year'\n"
            ),
        )

        G.add_edge("units", "final")
        G.add_node(
            "final",
            prompt=(
                "Please respond based on our entire conversation so far. "
                "Return your answer as a dictionary in "
                "JSON format (not markdown). Your JSON file must include "
                "exactly four keys. The keys are 'value', 'units', 'summary', "
                "and 'section'. The value of the '[value]' key "
                "should be a numerical value corresponding to the "
                "{restriction} for {tech}, or `null` if the text "
                "does not mention such a restriction. Use our conversation to "
                "fill out this value. The value of the 'units' key should be "
                "a string corresponding to the (standard) units for the "
                "{restriction} allowed for {tech} by the text "
                "below, or `null` if the text does not mention such a "
                "restriction. "
                "As before, focus only on {restriction} specifically for "
                "systems that would typically be defined as {tech} based on "
                "the text itself. "
                "{SUMMARY_PROMPT} {UNITS_IN_SUMMARY_PROMPT} {SECTION_PROMPT}"
            ),
        )
    elif "prohibitions" in kwargs.get("restriction", ""):
        G.add_edge(
            "init",
            "is_conditional",
            condition=llm_response_starts_with_yes,
        )
        G.add_node(
            "is_conditional",
            prompt=(
                "Does the prohibition, moratorium, or ban only apply "
                "conditionally? For example, does it only apply to those "
                "who have not complied with the provisions in this text? "
                "Please start your response with either 'Yes' or 'No' "
                "and briefly explain your answer."
            ),
        )
        G.add_edge(
            "is_conditional",
            "has_end_date",
            condition=llm_response_starts_with_no,
        )
        G.add_node(
            "has_end_date",
            prompt=(
                "Does the legal text given an expiration date for the "
                "prohibition, moratorium, or ban? "
                "Please start your response with either 'Yes' or 'No' "
                "and briefly explain your answer."
            ),
        )
        G.add_edge(
            "has_end_date", "final", condition=llm_response_starts_with_no
        )
        G.add_edge(
            "has_end_date",
            "check_end_date",
            condition=llm_response_starts_with_yes,
        )
        todays_date = datetime.now().strftime("%B %d, %Y")
        G.add_node(
            "check_end_date",
            prompt=(
                f"Today is {todays_date}. Has the prohibition, "
                "moratorium, or ban expired? "
                "Please start your response with either 'Yes' or 'No' "
                "and briefly explain your answer."
            ),
        )
        G.add_edge(
            "check_end_date",
            "final",
            condition=llm_response_starts_with_no,
        )
        G.add_node(
            "final",
            prompt=(
                "Please respond based on our entire conversation so far. "
                "Return your answer as a dictionary in "
                "JSON format (not markdown). Your JSON file must include "
                "exactly two keys. The keys are 'summary' and 'section'. "
                "{SUMMARY_PROMPT} If the prohibition is a moratorium, be "
                "sure to include that distinction in your summary and "
                "provide any relevant expiration dates. {SECTION_PROMPT}"
            ),
        )

    else:
        G.add_edge("init", "final", condition=llm_response_starts_with_yes)
        G.add_node(
            "final",
            prompt=(
                "Please respond based on our entire conversation so far. "
                "Return your answer as a dictionary in "
                "JSON format (not markdown). Your JSON file must include "
                "exactly two keys. The keys are 'summary' and 'section'. "
                "{SUMMARY_PROMPT} {SECTION_PROMPT}"
            ),
        )

    return G


def setup_graph_permitted_use_districts(**kwargs):
    """Setup graph to extract permitted use districts for technology

    Parameters
    ----------
    **kwargs
        Keyword-value pairs to add to graph.

    Returns
    -------
    nx.DiGraph
        Graph instance that can be used to initialize an
        `elm.tree.DecisionTree`.
    """
    G = setup_graph_no_nodes(**kwargs)  # noqa: N806

    G.add_node(
        "init",
        prompt=(
            "Does the following legal text explicitly define districts where "
            "{tech} (or similar) are permitted as {use_type}? {clarifications}"
            "Pay extra attention to titles and clarifying text found in "
            "parentheses and footnotes. Please start your response with "
            "either 'Yes' or 'No' and briefly explain your answer."
            '\n\n"""\n{text}\n"""'
        ),
    )
    G.add_edge(
        "init", "district_names", condition=llm_response_starts_with_yes
    )

    G.add_node(
        "district_names",
        prompt=(
            "What are all of the district names (and abbreviations if given) "
            "where {tech} (or similar) are permitted as {use_type}?"
        ),
    )
    G.add_edge("district_names", "final")

    G.add_node(
        "final",
        prompt=(
            "Please respond based on our entire conversation so far. "
            "Return your answer as a dictionary in "
            "JSON format (not markdown). Your JSON file must include "
            "exactly three keys. The keys are 'value', 'summary', "
            "and 'section'. The value of the 'value' key "
            "should be a list of all district names (and abbreviations if "
            "given) where {tech} (or similar) "
            "are permitted as {use_type}, or `null` if the text does not "
            "mention this use type for {tech} (or similar). Use our "
            "conversation to fill out this value. {SUMMARY_PROMPT} "
            "{SECTION_PROMPT}"
        ),
    )
    return G


class BaseTextExtractor:
    """Base implementation for a text extractor"""

    SYSTEM_MESSAGE = (
        "You are a text extraction assistant. Your job is to extract only "
        "verbatim, **unmodified** excerpts from provided legal or policy "
        "documents. Do not interpret or paraphrase. Do not summarize. Only "
        "return exactly copied segments that match the specified scope. If "
        "the relevant content appears within a space-delimited table, return "
        "the entire table, including headers and footers, exactly as "
        "formatted."
    )
    _USAGE_LABEL = LLMUsageCategory.DOCUMENT_ORDINANCE_SUMMARY

    def __init__(self, llm_caller):
        """

        Parameters
        ----------
        llm_caller : compass.llm.LLMCaller
            LLM Caller instance used to extract ordinance info with.
        """
        self.llm_caller = llm_caller

    async def _process(self, text_chunks, instructions, is_valid_chunk):
        """Perform extraction processing"""
        logger.info(
            "Extracting summary text from %d text chunks asynchronously...",
            len(text_chunks),
        )
        logger.debug("Model instructions are:\n%s", instructions)
        outer_task_name = asyncio.current_task().get_name()
        summaries = [
            asyncio.create_task(
                self.llm_caller.call(
                    sys_msg=self.SYSTEM_MESSAGE,
                    content=f"{instructions}\n\n# TEXT #\n\n{chunk}",
                    usage_sub_label=self._USAGE_LABEL,
                ),
                name=outer_task_name,
            )
            for chunk in text_chunks
        ]
        summary_chunks = await asyncio.gather(*summaries)
        summary_chunks = [
            clean_backticks_from_llm_response(chunk)
            for chunk in summary_chunks
            if is_valid_chunk(chunk)
        ]

        text_summary = merge_overlapping_texts(summary_chunks)
        logger.debug(
            "Final summary contains %d tokens",
            ApiBase.count_tokens(
                text_summary,
                model=self.llm_caller.kwargs.get("model", "gpt-4"),
            ),
        )
        return text_summary
