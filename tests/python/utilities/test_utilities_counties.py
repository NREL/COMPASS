"""COMPASS Ordinance jurisdiction utilities tests"""

from pathlib import Path

import pytest
import pandas as pd

from compass.utilities.counties import (
    load_all_jurisdiction_info,
    load_jurisdictions_from_fp,
    jurisdiction_websites,
)
from compass.exceptions import COMPASSValueError


def test_load_jurisdictions():
    """Test the `load_all_jurisdiction_info` function."""

    jurisdiction_info = load_all_jurisdiction_info()
    assert not jurisdiction_info.empty

    expected_cols = [
        "County",
        "State",
        "FIPS",
        "County Type",
        "Full Name",
        "Website",
    ]
    assert all(col in jurisdiction_info for col in expected_cols)
    assert len(jurisdiction_info) == len(
        jurisdiction_info.groupby(["County", "State"])
    )

    # Spot checks:
    assert "Decatur" in set(jurisdiction_info["County"])
    assert "Box Elder" in set(jurisdiction_info["County"])
    assert "Colorado" in set(jurisdiction_info["State"])
    assert "Rhode Island" in set(jurisdiction_info["State"])


def test_jurisdiction_websites():
    """Test the `jurisdiction_websites` function"""

    websites = jurisdiction_websites()
    assert len(websites) == len(load_all_jurisdiction_info())
    assert isinstance(websites, dict)
    assert all(isinstance(key, tuple) for key in websites)
    assert all(len(key) == 2 for key in websites)

    # Spot checks:
    assert ("decatur", "indiana") in websites
    assert ("el paso", "colorado") in websites
    assert ("box elder", "utah") in websites


def test_load_jurisdictions_from_fp(tmp_path):
    """Test `load_jurisdictions_from_fp` function."""

    test_jurisdiction_fp = tmp_path / "out.csv"
    input_counties = pd.DataFrame(
        {"County": ["decatur", "DNE County"], "State": ["INDIANA", "colorado"]}
    )
    input_counties.to_csv(test_jurisdiction_fp)

    counties = load_jurisdictions_from_fp(test_jurisdiction_fp)

    assert len(counties) == 1
    assert set(counties["County"]) == {"Decatur"}
    assert set(counties["State"]) == {"Indiana"}
    assert {type(val) for val in counties["FIPS"]} == {int}


def test_load_jurisdictions_from_fp_bad_input(tmp_path):
    """Test `load_jurisdictions_from_fp` function."""

    test_jurisdiction_fp = tmp_path / "out.csv"
    pd.DataFrame().to_csv(test_jurisdiction_fp)

    with pytest.raises(COMPASSValueError) as err:
        load_jurisdictions_from_fp(test_jurisdiction_fp)

    expected_msg = (
        "The following required columns were not found in the "
        "jurisdiction input:"
    )
    assert expected_msg in str(err)
    assert "County" in str(err)
    assert "State" in str(err)


if __name__ == "__main__":
    pytest.main(["-q", "--show-capture=all", Path(__file__), "-rapP"])
