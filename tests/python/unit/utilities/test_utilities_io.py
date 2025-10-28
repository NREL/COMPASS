"""Test COMPASS I/O utilities"""

from pathlib import Path

import pytest

from compass.utilities.io import load_local_docs


@pytest.mark.asyncio
async def test_basic_load_pdf(test_data_files_dir):
    """Test basic loading of local PDF document"""
    test_fp = test_data_files_dir / "Caneadea New York.pdf"

    docs = await load_local_docs([test_fp])
    assert len(docs) == 1
    doc = docs[0]
    assert not doc.empty
    assert Path(doc.attrs.get("source_fp")) == test_fp
    assert len(doc.pages) == 3


@pytest.mark.asyncio
async def test_basic_load_html(test_data_files_dir):
    """Test basic loading of local HTML document"""
    test_fp = test_data_files_dir / "Whatcom.txt"

    docs = await load_local_docs([test_fp])
    assert len(docs) == 1
    doc = docs[0]
    assert not doc.empty
    assert Path(doc.attrs.get("source_fp")) == test_fp
    assert len(doc.pages) == 1


if __name__ == "__main__":
    pytest.main(["-q", "--show-capture=all", Path(__file__), "-rapP"])
