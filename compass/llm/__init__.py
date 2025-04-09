"""COMPASS Ordinance LLM callers"""

from enum import StrEnum, auto

from .calling import (
    LLMCaller,
    ChatLLMCaller,
    StructuredLLMCaller,
    LLMCallerArgs,
)


class LLMTasks(StrEnum):
    """LLM-based COMPASS tasks"""

    CHAT = auto()
    DATE_EXTRACTION = auto()
    DECISION_TREE = auto()
    DEFAULT = auto()
    DOCUMENT_CONTENT_VALIDATION = auto()
    DOCUMENT_ORDINANCE_SUMMARY = auto()
    DOCUMENT_PERMITTED_USE_CONTENT_VALIDATION = auto()
    DOCUMENT_LOCATION_VALIDATION = auto()
