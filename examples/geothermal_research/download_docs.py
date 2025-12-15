"""Example on downloading Ordinance docs"""

import logging
import asyncio
import warnings
from pathlib import Path

from rich.theme import Theme
from rich.logging import RichHandler
from rich.console import Console

from compass.services.provider import RunningAsyncServices
from compass.services.threaded import TempFileCachePB
from compass.services.cpu import PDFLoader, read_pdf_doc
from compass.utilities.logs import AddLocationFilter
from compass.utilities.location import Jurisdiction
from compass.scripts.download import (
    download_jurisdiction_ordinance_using_search_engine,
)
from compass.pb import COMPASS_PB


logger = logging.getLogger("compass")


def _setup_logging(log_level="INFO"):
    """Setup logging"""
    custom_theme = Theme({"logging.level.trace": "rgb(94,79,162)"})
    console = Console(theme=custom_theme)

    handler = RichHandler(
        console=console,
        rich_tracebacks=True,
        omit_repeated_times=True,
        markup=True,
    )
    fmt = logging.Formatter(
        fmt="[[magenta]%(location)s[/magenta]]: %(message)s",
        defaults={"location": "main"},
    )
    handler.setFormatter(fmt)
    handler.addFilter(AddLocationFilter())
    logger.addHandler(handler)
    logger.setLevel(log_level)


async def _download_docs(question_templates, jurisdiction, num_urls):
    logger.info("Searching for documents...")

    services = [TempFileCachePB(), PDFLoader()]
    async with RunningAsyncServices(services):
        return await download_jurisdiction_ordinance_using_search_engine(
            question_templates,
            jurisdiction,
            num_urls=num_urls,
            search_engines=["SerpAPIGoogleSearch"],
            google_serpapi_kwargs={
                "api_key": (
                    "54d186df05c67b1e8ed1e50e07a188186a63a41021d1d90b202282c3d5b0fd7a"
                ),
                "verify": False,
            },
            file_loader_kwargs={
                "pdf_read_coroutine": read_pdf_doc,
                "verify_ssl": False,
            }
        )


if __name__ == "__main__":
    _setup_logging(log_level="DEBUG")
    warnings.filterwarnings("ignore")

    question_templates = [
        # Target official county websites with land use codes
        (
            'site:colorado.gov OR site:chaffee.co.us '
            '"land use code" geothermal filetype:pdf'
        ),
        '"{jurisdiction}" "land use code" geothermal filetype:pdf',

        # Search for specific chapter/guideline titles
        (
            '"{jurisdiction}" "Guidelines and Regulations for the '
            'Use of Geothermal" filetype:pdf'
        ),
        (
            '"{jurisdiction}" "geothermal electricity" regulations '
            'filetype:pdf'
        ),

        # Search county planning/zoning department sites
        (
            "site:chaffeecounty.org planning zoning geothermal "
            "filetype:pdf"
        ),
        (
            '"{jurisdiction}" planning zoning "land use code" '
            'filetype:pdf'
        ),

        # Original broad searches as fallback
        'filetype:pdf "{jurisdiction}" geothermal ordinance',
        'filetype:pdf "{jurisdiction}" land use code geothermal',
        '"{jurisdiction}" geothermal code regulations',
        '"{jurisdiction}" geothermal development standards',
    ]
    jurisdiction = Jurisdiction(
        subdivision_type="county", state="Colorado", county="Chaffee"
    )

    output_dir = (
        Path("downloaded_docs") / jurisdiction.state / jurisdiction.county
    )
    output_dir.mkdir(parents=True, exist_ok=True)
    logger.info("Saving documents to: %s", output_dir)

    COMPASS_PB.create_main_task(num_jurisdictions=1)
    num_urls = 5
    with (
        COMPASS_PB.jurisdiction_prog_bar(jurisdiction.full_name),
    ):
        docs = asyncio.run(
            _download_docs(question_templates, jurisdiction, num_urls)
        )

    logger.info("\nFound %d documents:", len(docs))
    for i, doc in enumerate(docs, 1):
        source = doc.attrs.get("source", "Unknown source")
        title = doc.attrs.get("title", "No title")
        logger.info("\n%d. %s", i, source)
        logger.info("   Title: %s", title)

        if hasattr(doc, "text") and doc.text:
            safe_filename = f"doc_{i}_{jurisdiction.county}.pdf"
            save_path = output_dir / safe_filename
            try:
                save_path.write_bytes(doc.text.encode("utf-8"))
                logger.info("   Saved to: %s", save_path)
            except OSError as e:
                logger.warning("   Could not save file: %s", e)
