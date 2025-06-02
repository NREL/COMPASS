"""Document validation decision tree graph setup functions"""

from compass.extraction.common import (
    setup_graph_no_nodes,
    llm_response_starts_with_yes,
    llm_response_starts_with_no,
)


def setup_graph_correct_jurisdiction_type(jurisdiction, **kwargs):
    """Setup graph to check for correct jurisdiction type in legal text

    Parameters
    ----------
    jurisdiction : compass.utilities.location.Jurisdiction
        Jurisdiction for which validation is being performed.
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
            "Does the following legal text explicitly outline the type of "
            "jurisdiction it applies to? Common types of jurisdictions "
            "include 'state', 'county', 'city', 'township',' borough', etc. "
            "Begin your response with either 'Yes' or 'No' and explain your "
            "answer."
            '\n\n"""\n{text}\n"""'
        ),
    )

    G.add_edge("init", "is_state", condition=llm_response_starts_with_yes)
    G.add_node(
        "is_state",
        prompt=(
            "Does the legal text explicitly state that the statutes within "
            f"apply to **the entire area** governed by {jurisdiction.state} "
            "state? If the legal text applies to a different state or only to "
            "a subdivision like a county or township within "
            f"{jurisdiction.state} state, say 'No'. "
            "Begin your response with either 'Yes' or 'No' and explain your "
            "answer."
        ),
    )
    node_to_connect = "is_state"

    if jurisdiction.county:
        G.add_edge(
            node_to_connect, "is_county", condition=llm_response_starts_with_no
        )
        G.add_edge(
            node_to_connect, "final", condition=llm_response_starts_with_yes
        )
        G.add_node(
            "is_county",
            prompt=(
                "Does the legal text explicitly state that the statutes "
                "within apply to **the entire area** governed by "
                f"{jurisdiction.full_county_phrase}? If the legal text "
                "applies to a different county or only to a subdivision like "
                "a township or borough within "
                f"{jurisdiction.full_county_phrase}, say 'No'. "
                "Begin your response with either 'Yes' or 'No' and explain "
                "your answer."
            ),
        )
        G.add_edge(
            "is_county", "final", condition=llm_response_starts_with_yes
        )
        node_to_connect = "is_county"

    if jurisdiction.subdivision_name:
        G.add_edge(
            node_to_connect, "is_city", condition=llm_response_starts_with_no
        )
        G.add_edge(
            node_to_connect, "final", condition=llm_response_starts_with_yes
        )
        G.add_node(
            "is_city",
            prompt=(
                "Does the legal text explicitly state that the statutes "
                "within apply to **the entire area** governed by the"
                f"{jurisdiction.full_subdivision_phrase}? If the legal text "
                "applies to a different city, township, etc, say 'No'. "
                "Begin your response with either 'Yes' or 'No' and explain "
                "your answer."
            ),
        )
        node_to_connect = "is_city"

    G.add_edge(node_to_connect, "final")
    G.add_node(
        "final",
        prompt=(
            "Respond based on our entire conversation so far. Return your "
            "answer as a dictionary in JSON format (not markdown). Your JSON "
            "file must include exactly two keys. The keys are "
            "'correct_jurisdiction' and 'explanation'. The value of the "
            "'correct_jurisdiction' key should be a boolean that is set to "
            "`true` **only if** the text explicitly states that that the "
            "statutes within apply to **the entire area** governed by "
            f"{jurisdiction.full_name} (`false` otherwise). The value of the "
            "'explanation' key should be a string containing a short "
            "explanation for your choice. "
        ),
    )
    return G
