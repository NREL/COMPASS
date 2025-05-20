"""COMPASS Ordinance Location utility tests"""

from pathlib import Path

import pytest

from compass.utilities.location import Jurisdiction


def test_basic_state_properties():
    """Test basic properties for ``Jurisdiction`` class for a state"""

    state = Jurisdiction("state", state="Colorado")

    assert repr(state) == "Colorado"
    assert state.full_name == "Colorado"
    assert state.full_name == str(state)

    assert state == Jurisdiction("state", state="cOlORAdo")
    assert state != Jurisdiction("city", state="Colorado")

    assert state == "Colorado"
    assert state == "colorado"


def test_basic_county_properties():
    """Test basic properties for ``Jurisdiction`` class for a county"""

    county = Jurisdiction("county", county="Box Elder", state="Utah")

    assert repr(county) == "Box Elder County, Utah"
    assert county.full_name == "Box Elder County, Utah"
    assert county.full_name == str(county)

    assert county == Jurisdiction("county", county="Box elder", state="uTah")
    assert county != Jurisdiction("city", county="Box Elder", state="Utah")

    assert county == "Box Elder County, Utah"
    assert county == "Box elder county, Utah"


@pytest.mark.parametrize("jt", ["town", "city", "borough"])
def test_basic_town_properties(jt):
    """Test basic properties for ``Jurisdiction`` class for a town"""

    town = Jurisdiction(
        jt, county="Jefferson", state="Colorado", subdivision_name="Golden"
    )

    assert repr(town) == f"{jt.title()} of Golden, Jefferson County, Colorado"
    assert (
        town.full_name == f"{jt.title()} of Golden, Jefferson County, Colorado"
    )
    assert town.full_name == str(town)

    assert town == Jurisdiction(
        jt, county="jefferson", state="colorado", subdivision_name="golden"
    )
    assert town != Jurisdiction(
        "county",
        county="Jefferson",
        state="Colorado",
        subdivision_name="Golden",
    )

    assert town == f"{jt.title()} of Golden, Jefferson County, Colorado"
    assert town == f"{jt.title()} of golden, jefferson county, colorado"


def test_atypical_subdivision_properties():
    """Test basic properties for ``Jurisdiction`` class for a subdivision"""

    gore = Jurisdiction(
        "gore", county="Chittenden", state="Vermont", subdivision_name="Buels"
    )

    assert repr(gore) == "Buels Gore, Chittenden County, Vermont"
    assert gore.full_name == "Buels Gore, Chittenden County, Vermont"
    assert gore.full_name == str(gore)

    assert gore == Jurisdiction(
        "gore", county="chittenden", state="vermont", subdivision_name="buels"
    )
    assert gore != Jurisdiction(
        "county",
        county="Chittenden",
        state="Vermont",
        subdivision_name="Buels",
    )

    assert gore == "Buels Gore, Chittenden County, Vermont"
    assert gore == "buels gOre, chittENden county, vermonT"


def test_city_no_county():
    """Test ``Jurisdiction`` for a city with no county"""

    gore = Jurisdiction("city", "Maryland", subdivision_name="Baltimore")

    assert repr(gore) == "City of Baltimore, Maryland"
    assert gore.full_name == "City of Baltimore, Maryland"
    assert gore.full_name == str(gore)

    assert gore == Jurisdiction(
        "city", "maryland", subdivision_name="baltimore"
    )
    assert gore != Jurisdiction(
        "county", "maryland", subdivision_name="baltimore"
    )

    assert gore == "City of Baltimore, Maryland"
    assert gore == "ciTy of baltiMore, maryland"


if __name__ == "__main__":
    pytest.main(["-q", "--show-capture=all", Path(__file__), "-rapP"])
