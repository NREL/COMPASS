"""Tests for compass.scripts.process"""

import logging
from pathlib import Path

import pytest

from compass.exceptions import COMPASSValueError
import compass.scripts.process as process_module
from compass.scripts.process import (
    _COMPASSRunner,
    process_jurisdictions_with_openai,
)
from compass.utilities import ProcessKwargs


@pytest.fixture
def testing_log_file(tmp_path):
    """Logger fixture for testing"""
    log_fp = tmp_path / "test.log"
    handler = logging.FileHandler(log_fp, encoding="utf-8")
    logger = logging.getLogger("compass")
    prev_level = logger.level
    prev_propagate = logger.propagate
    logger.setLevel(logging.ERROR)
    logger.propagate = False
    logger.addHandler(handler)

    yield log_fp

    handler.flush()
    logger.removeHandler(handler)
    handler.close()
    logger.setLevel(prev_level)
    logger.propagate = prev_propagate


def test_known_local_docs_missing_file(tmp_path):
    """Raise when known_local_docs points to missing config"""
    missing_fp = tmp_path / "does_not_exist.json"
    runner = _COMPASSRunner(
        dirs=None,
        log_listener=None,
        tech="solar",
        models={},
        process_kwargs=ProcessKwargs(str(missing_fp), None),
    )

    with pytest.raises(COMPASSValueError, match="Config file does not exist"):
        _ = runner.known_local_docs


def test_known_local_docs_logs_missing_file(tmp_path, testing_log_file):
    """Log missing known_local_docs config to error file"""

    missing_fp = tmp_path / "does_not_exist.json"
    runner = _COMPASSRunner(
        dirs=None,
        log_listener=None,
        tech="solar",
        models={},
        process_kwargs=ProcessKwargs(str(missing_fp), None),
    )

    with pytest.raises(COMPASSValueError, match="Config file does not exist"):
        _ = runner.known_local_docs

    assert testing_log_file.exists()
    assert "Config file does not exist" in testing_log_file.read_text(
        encoding="utf-8"
    )


@pytest.mark.asyncio
async def test_duplicate_tasks_logs_to_file(tmp_path):
    """Log duplicate LLM tasks to error file"""

    jurisdiction_fp = tmp_path / "jurisdictions.csv"
    jurisdiction_fp.touch()

    with pytest.raises(COMPASSValueError, match="Found duplicated task"):
        _ = await process_jurisdictions_with_openai(
            out_dir=tmp_path / "outputs",
            tech="solar",
            jurisdiction_fp=jurisdiction_fp,
            model=[
                {
                    "name": "gpt-4.1-mini",
                    "tasks": ["default", "date_extraction"],
                },
                {
                    "name": "gpt-4.1",
                    "tasks": [
                        "ordinance_text_extraction",
                        "permitted_use_text_extraction",
                        "date_extraction",
                    ],
                },
            ],
        )

    log_files = list((tmp_path / "outputs" / "logs").glob("*"))
    assert len(log_files) == 1
    assert "Fatal error during processing" not in log_files[0].read_text(
        encoding="utf-8"
    )
    assert "Found duplicated task" in log_files[0].read_text(encoding="utf-8")


@pytest.mark.asyncio
async def test_external_exceptions_logged_to_file(tmp_path, monkeypatch):
    """Log external exceptions to error file"""

    def _always_fail(*__, **___):
        raise NotImplementedError("Simulated external error")

    monkeypatch.setattr(
        process_module, "_initialize_model_params", _always_fail
    )

    jurisdiction_fp = tmp_path / "jurisdictions.csv"
    jurisdiction_fp.touch()

    with pytest.raises(NotImplementedError, match="Simulated external error"):
        _ = await process_jurisdictions_with_openai(
            out_dir=tmp_path / "outputs",
            tech="solar",
            jurisdiction_fp=jurisdiction_fp,
        )

    log_files = list((tmp_path / "outputs" / "logs").glob("*"))
    assert len(log_files) == 1
    assert "Fatal error during processing" in log_files[0].read_text(
        encoding="utf-8"
    )
    assert "Simulated external error" in log_files[0].read_text(
        encoding="utf-8"
    )


if __name__ == "__main__":
    pytest.main(["-q", "--show-capture=all", Path(__file__), "-rapP"])
