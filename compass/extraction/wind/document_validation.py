"""Ordinance document validation logic for wind"""

NOT_WIND_WORDS = [
    "windy",
    "winds",
    "window",
    "windiest",
    "windbreak",
    "windshield",
    "wind blow",
    "wind erosion",
    "rewind",
    "mini wecs",
    "swecs",
    "private wecs",
    "pwecs",
    "wind direction",
    "wind movement",
    "wind attribute",
    "wind runway",
    "wind load",
    "wind orient",
    "wind damage",
]
GOOD_WIND_KEYWORDS = ["wind", "setback"]
GOOD_WIND_ACRONYMS = ["wecs", "wes", "lwet", "uwet", "wef"]
_GOOD_ACRONYM_CONTEXTS = [
    " {acronym} ",
    " {acronym}\n",
    " {acronym}.",
    "\n{acronym} ",
    "\n{acronym}.",
    "\n{acronym}\n",
    "({acronym} ",
    " {acronym})",
]
GOOD_WIND_PHRASES = ["wind energy conversion", "wind turbine", "wind tower"]


def possibly_mentions_wind(text, match_count_threshold=1):
    """Perform a heuristic check for mention of wind energy in text

    This check first strips the text of any wind "look-alike" words
    (e.g. "window", "windshield", etc). Then, it checks for particular
    keywords, acronyms, and phrases that pertain to wind in the text.
    If enough keywords are mentions (as dictated by
    `match_count_threshold`), this check returns ``True``.

    Parameters
    ----------
    text : str
        Input text that may or may not mention win in relation to wind
        energy.
    match_count_threshold : int, optional
        Number of keywords that must match for the text to pass this
        heuristic check. Count must be strictly greater than this value.
        By default, ``1``.

    Returns
    -------
    bool
        ``True`` if the number of keywords/acronyms/phrases detected
        exceeds the `match_count_threshold`.
    """
    heuristics_text = _convert_to_heuristics_text(text)
    total_keyword_matches = _count_single_keyword_matches(heuristics_text)
    total_keyword_matches += _count_acronym_matches(heuristics_text)
    total_keyword_matches += _count_phrase_matches(heuristics_text)
    return total_keyword_matches > match_count_threshold


def _convert_to_heuristics_text(text):
    """Convert text for heuristic wind content parsing"""
    heuristics_text = text.casefold()
    for word in NOT_WIND_WORDS:
        heuristics_text = heuristics_text.replace(word, "")
    return heuristics_text


def _count_single_keyword_matches(heuristics_text):
    """Count number of good wind energy keywords that appear in text"""
    return sum(keyword in heuristics_text for keyword in GOOD_WIND_KEYWORDS)


def _count_acronym_matches(heuristics_text):
    """Count number of good wind energy acronyms that appear in text"""
    acronym_matches = 0
    for context in _GOOD_ACRONYM_CONTEXTS:
        acronym_keywords = {
            context.format(acronym=acronym) for acronym in GOOD_WIND_ACRONYMS
        }
        acronym_matches = sum(
            keyword in heuristics_text for keyword in acronym_keywords
        )
        if acronym_matches > 0:
            break
    return acronym_matches


def _count_phrase_matches(heuristics_text):
    """Count number of good wind energy phrases that appear in text"""
    return sum(
        all(keyword in heuristics_text for keyword in phrase.split(" "))
        for phrase in GOOD_WIND_PHRASES
    )
