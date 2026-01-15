"""COMPASS ordinance validation tests"""

from pathlib import Path

import pytest

from compass.extraction.wind.ordinance import WindHeuristic
from compass.extraction.solar.ordinance import SolarHeuristic
from compass.extraction.geothermal_electricity.ordinance import (
    GeothermalElectricityHeuristic,
)


@pytest.mark.parametrize(
    "text,truth",
    [
        ("Wind SETBACKS", True),
        (" WECS SETBACKS", True),
        ("Window SETBACKS", False),
        ("SWECS SETBACKS", False),
        ("(wind LWET)", True),
        ("(wind\n LWET)", True),
        ("Wind SWECS", False),
        ("Wind WES", False),
        ("Wind WES\n", True),
        ("wind turbines and wind towers", True),
    ],
)
def test_possibly_mentions_wind(text, truth):
    """Test for `WindHeuristic` class (basic execution)"""

    assert WindHeuristic().check(text) == truth


@pytest.mark.parametrize(
    "text,truth",
    [
        ("Solar SETBACKS", True),
        (" SECS SETBACKS", True),
        ("SOLARIS SETBACKS", False),
        ("WECS SETBACKS", False),
        ("(solar farm)", True),
        ("(solar\nfarm)", True),
        ("Solar WECS", False),
        ("Solar SES", False),
        ("Solar SES\n", True),
        ("solar panels and solar farms", True),
    ],
)
def test_possibly_mentions_solar(text, truth):
    """Test for `SolarHeuristic` class (basic execution)"""

    assert SolarHeuristic().check(text) == truth


@pytest.mark.parametrize(
    "text,truth",
    [
        ("Geothermal power plant SETBACKS", True),
        ("geothermal electricity generation SETBACKS", True),
        ("geothermal facility setbacks", True),
        ("geothermal turbine requirements", True),
        ("geothermal generator spacing", True),
        ("geothermal wellfield setbacks", True),
        ("geothermal production well setbacks", True),
        ("geothermal power production", True),
        ("geothermal electricity generation", True),
        ("geothermal generating facilities", True),
        ("geothermal overlay zone", True),
        ("geothermal heat pump SETBACKS", False),
        ("ground source heat pump requirements", False),
        ("GSHP setbacks", False),
        ("residential geothermal systems", False),
        ("geothermal HVAC requirements", False),
        ("ground-coupled heat pump", False),
        ("geoexchange system regulations", False),
        ("closed loop geothermal", False),
        ("space heating geothermal", False),
        ("district heating geothermal", False),
        ("greenhouse heating geothermal", False),
        ("accessory geothermal use", False),
        ("geothermal direct use", False),
        ("geothermal setback turbine", True),
        ("geothermal\npower plant", True),
        ("geothermal wellfield generator", True),
    ],
)
def test_possibly_mentions_geothermal_electricity(text, truth):
    """Test for `GeothermalElectricityHeuristic` class (basic execution)"""

    assert GeothermalElectricityHeuristic().check(text) == truth


if __name__ == "__main__":
    pytest.main(["-q", "--show-capture=all", Path(__file__), "-rapP"])
