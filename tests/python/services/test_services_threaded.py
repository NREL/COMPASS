"""Test COMPASS Ordinances TempFileCache Services"""

from pathlib import Path
from datetime import datetime

import pytest

from elm.web.document import HTMLDocument
from compass.services.threaded import TempFileCache, FileMover


@pytest.mark.asyncio
async def test_temp_file_cache_service():
    """Test base implementation of `TempFileCache` class"""

    doc = HTMLDocument(["test"])
    doc.attrs["source"] = "http://www.example.com/?=%20test"

    cache = TempFileCache()
    cache.acquire_resources()
    out_fp = await cache.process(doc, doc.text)
    assert out_fp.exists()
    assert out_fp.read_text().startswith("test")
    cache.release_resources()
    assert not out_fp.exists()


@pytest.mark.asyncio
async def test_file_move_service(tmp_path):
    """Test base implementation of `FileMover` class"""

    doc = HTMLDocument(["test"])
    doc.attrs["source"] = "http://www.example.com/?=%20test"

    cache = TempFileCache()
    cache.acquire_resources()
    out_fp = await cache.process(doc, doc.text)
    assert out_fp.exists()
    assert out_fp.read_text().startswith("test")
    doc.attrs["cache_fn"] = out_fp

    date = datetime.now().strftime("%Y_%m_%d")
    expected_moved_fp = tmp_path / f"{out_fp.stem}_downloaded_{date}.txt"
    assert not expected_moved_fp.exists()
    mover = FileMover(tmp_path)
    mover.acquire_resources()
    moved_fp = await mover.process(doc)
    assert expected_moved_fp == moved_fp
    assert not out_fp.exists()
    assert moved_fp.exists()
    assert moved_fp.read_text().startswith("test")

    cache.release_resources()
    mover.release_resources()
    assert moved_fp.exists()


if __name__ == "__main__":
    pytest.main(["-q", "--show-capture=all", Path(__file__), "-rapP"])
