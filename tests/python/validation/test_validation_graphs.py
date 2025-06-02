"""COMPASS Ordinance content validation tests"""

from pathlib import Path

import pytest

from compass.utilities.location import Jurisdiction
from compass.validation.graphs import setup_graph_correct_jurisdiction_type


def test_setup_graph_correct_jurisdiction_type_state():
    """Test setting up jurisdiction validation graph for state"""
    loc = Jurisdiction("state", state="New York")
    graph = setup_graph_correct_jurisdiction_type(loc)

    assert set(graph.nodes) == {"init", "is_state", "final"}
    assert list(graph.edges) == [("init", "is_state"), ("is_state", "final")]

    assert f"{loc.state} state" in graph.nodes["is_state"]["prompt"]
    assert f"{loc.full_name}" in graph.nodes["final"]["prompt"]


@pytest.mark.parametrize("county_type", ["parish", "county"])
def test_setup_graph_correct_jurisdiction_type_county(county_type):
    """Test setting up jurisdiction validation graph for county"""

    loc = Jurisdiction(county_type, state="New York", county="Test")
    graph = setup_graph_correct_jurisdiction_type(loc)

    assert set(graph.nodes) == {"init", "is_state", "is_county", "final"}
    assert list(graph.edges) == [
        ("init", "is_state"),
        ("is_state", "is_county"),
        ("is_state", "final"),
        ("is_county", "final"),
    ]

    assert f"{loc.state} state" in graph.nodes["is_state"]["prompt"]
    assert f"{loc.full_county_phrase}" in graph.nodes["is_county"]["prompt"]
    assert f"{loc.full_name}" in graph.nodes["final"]["prompt"]


def test_setup_graph_correct_jurisdiction_type_city_no_county():
    """Test setting up jurisdiction validation graph for city no county"""

    loc = Jurisdiction("city", state="New York", subdivision_name="test")
    graph = setup_graph_correct_jurisdiction_type(loc)

    assert set(graph.nodes) == {"init", "is_state", "is_city", "final"}
    assert list(graph.edges) == [
        ("init", "is_state"),
        ("is_state", "is_city"),
        ("is_state", "final"),
        ("is_city", "final"),
    ]

    assert f"{loc.state} state" in graph.nodes["is_state"]["prompt"]
    assert f"{loc.full_subdivision_phrase}" in graph.nodes["is_city"]["prompt"]
    assert f"{loc.full_name}" in graph.nodes["final"]["prompt"]


def test_setup_graph_correct_jurisdiction_type_city():
    """Test setting up jurisdiction validation graph for city"""

    loc = Jurisdiction(
        "city", state="Colorado", county="Jefferson", subdivision_name="Golden"
    )
    graph = setup_graph_correct_jurisdiction_type(loc)

    assert set(graph.nodes) == {
        "init",
        "is_state",
        "is_county",
        "is_city",
        "final",
    }
    assert list(graph.edges) == [
        ("init", "is_state"),
        ("is_state", "is_county"),
        ("is_state", "final"),
        ("is_county", "final"),
        ("is_county", "is_city"),
        ("is_city", "final"),
    ]

    assert f"{loc.state} state" in graph.nodes["is_state"]["prompt"]
    assert f"{loc.full_county_phrase}" in graph.nodes["is_county"]["prompt"]
    assert f"{loc.full_subdivision_phrase}" in graph.nodes["is_city"]["prompt"]
    assert f"{loc.full_name}" in graph.nodes["final"]["prompt"]


if __name__ == "__main__":
    pytest.main(["-q", "--show-capture=all", Path(__file__), "-rapP"])
