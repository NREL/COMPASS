"""COMPASS Ordinance content validation tests"""

from pathlib import Path

import pytest

from compass.validation.content import ParseChunksWithMemory


@pytest.mark.asyncio
async def test_validation_with_mem():
    """Test the `ParseChunksWithMemory` class (basic execution)"""

    keys = []

    class MockStructuredLLMCaller:
        """Mock LLM caller for tests."""

        async def call(self, key, text_chunk):
            """Mock LLM call and record system message"""
            keys.append(key)
            return text_chunk == 0

    text_chunks = list(range(7))
    validator = ParseChunksWithMemory(text_chunks, 3)
    caller = MockStructuredLLMCaller()

    out = await validator.parse_from_ind(
        0, key="test", llm_call_callback=caller.call
    )
    assert out
    assert keys == ["test"]
    assert validator.memory == [{"test": True}, {}, {}, {}, {}, {}, {}]

    out = await validator.parse_from_ind(
        2, key="test", llm_call_callback=caller.call
    )
    assert out
    assert keys == ["test"] * 3
    assert validator.memory == [
        {"test": True},
        {"test": False},
        {"test": False},
        {},
        {},
        {},
        {},
    ]

    out = await validator.parse_from_ind(
        6, key="test", llm_call_callback=caller.call
    )
    assert not out
    assert keys == ["test"] * 6
    assert validator.memory == [
        {"test": True},
        {"test": False},
        {"test": False},
        {},
        {"test": False},
        {"test": False},
        {"test": False},
    ]


if __name__ == "__main__":
    pytest.main(["-q", "--show-capture=all", Path(__file__), "-rapP"])
