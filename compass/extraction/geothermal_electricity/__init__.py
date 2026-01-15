"""Geothermal electricity generation ordinance extraction utilities"""

from .ordinance import (
    GeothermalElectricityHeuristic,
    GeothermalElectricityOrdinanceTextCollector,
    GeothermalElectricityOrdinanceTextExtractor,
    GeothermalElectricityPermittedUseDistrictsTextCollector,
    GeothermalElectricityPermittedUseDistrictsTextExtractor,
)


GEOTHERMAL_ELECTRICITY_QUESTION_TEMPLATES = [
    "filetype:pdf {jurisdiction} geothermal power plant ordinance",
    "geothermal electricity generation ordinance {jurisdiction}",
    "{jurisdiction} geothermal energy facility zoning ordinance",
    (
        "Where can I find the legal text for geothermal power plant "
        "zoning ordinances in {jurisdiction}?"
    ),
    (
        "What is the specific legal information regarding zoning "
        "ordinances for geothermal electricity generation facilities in "
        "{jurisdiction}?"
    ),
]

BEST_GEOTHERMAL_ELECTRICITY_ORDINANCE_WEBSITE_URL_KEYWORDS = {
    "pdf": 92160,
    "geothermal": 46080,
    "ordinance": 23040,
    "zoning": 11520,
    "regulation": 5760,
    "code": 2880,
    "power": 1440,
    "electricity": 1440,
    "planning": 720,
    "government": 180,
}
