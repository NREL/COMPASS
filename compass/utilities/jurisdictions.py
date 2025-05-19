"""Ordinance jurisdiction info"""

import logging
from warnings import warn
from pathlib import Path

import pandas as pd

from compass.exceptions import COMPASSValueError
from compass.warn import COMPASSWarning


logger = logging.getLogger(__name__)
_COUNTY_DATA_FP = Path(__file__).parent.parent / "data" / "conus_counties.csv"


def load_all_jurisdiction_info():
    """Load DataFrame containing info for all jurisdictions

    Returns
    -------
    pd.DataFrame
        DataFrame containing info like names, FIPS, websites, etc. for
        all jurisdictions.
    """
    jurisdiction_info = pd.read_csv(_COUNTY_DATA_FP)
    jurisdiction_info = _convert_to_title(jurisdiction_info, "County")
    return _convert_to_title(jurisdiction_info, "State")


def jurisdiction_websites(jurisdiction_info=None):
    """Load mapping of jurisdiction name and state to website

    Parameters
    ----------
    jurisdiction_info : pd.DataFrame, optional
        DataFrame containing jurisdiction names and websites. If
        ``None``, this info is loaded using
        :func:`load_jurisdiction_info`.
        By default, ``None``.

    Returns
    -------
    dict
        Dictionary where keys are tuples of (county, state) and keys are
        the relevant website URL. Note that county and state names are
        lowercase.
    """
    if jurisdiction_info is None:
        jurisdiction_info = load_all_jurisdiction_info()

    return {
        (row["County"].casefold(), row["State"].casefold()): row["Website"]
        for __, row in jurisdiction_info.iterrows()
    }


def load_jurisdictions_from_fp(jurisdiction_fp):
    """Load jurisdiction info base don counties in the input fp

    Parameters
    ----------
    jurisdiction_fp : path-like
        Path to csv file containing "County" and "State" columns that
        define the counties for which info should be loaded.

    Returns
    -------
    pd.DataFrame
        DataFrame containing jurisdiction info like names, FIPS,
        websites, etc. for all requested jurisdictions (that were
        found).
    """
    jurisdictions = pd.read_csv(jurisdiction_fp)
    _validate_jurisdiction_input(jurisdictions)

    jurisdictions = _convert_to_title(jurisdictions, "County")
    jurisdictions = _convert_to_title(jurisdictions, "State")

    all_jurisdiction_info = load_all_jurisdiction_info()
    jurisdictions = jurisdictions.merge(
        all_jurisdiction_info, on=["County", "State"], how="left"
    )

    jurisdictions = _filter_not_found_jurisdictions(jurisdictions)
    return _format_jurisdiction_df_for_output(jurisdictions)


def _validate_jurisdiction_input(df):
    """Throw error if user is missing required columns"""
    expected_cols = ["County", "State"]
    missing = [col for col in expected_cols if col not in df]
    if missing:
        msg = (
            "The following required columns were not found in the "
            f"jurisdiction input: {missing}"
        )
        raise COMPASSValueError(msg)


def _filter_not_found_jurisdictions(df):
    """Filter out jurisdictions with null FIPS codes"""
    _warn_about_missing_jurisdictions(df)
    return df[~df.FIPS.isna()].copy()


def _warn_about_missing_jurisdictions(df):
    """Throw warning about jurisdictions that were not in the list"""
    not_found_jurisdictions = df[df.FIPS.isna()]
    if len(not_found_jurisdictions):
        not_found_jurisdictions_str = not_found_jurisdictions[
            ["County", "State"]
            # cspell: disable-next-line
        ].to_markdown(index=False, tablefmt="psql")
        msg = (
            "The following jurisdictions were not found! Please make sure to "
            "use proper spelling and capitalization.\n"
            f"{not_found_jurisdictions_str}"
        )
        warn(msg, COMPASSWarning)


def _format_jurisdiction_df_for_output(df):
    """Format jurisdiction DataFrame for output"""
    out_cols = ["County", "State", "County Type", "FIPS", "Website"]
    df.FIPS = df.FIPS.astype(int)
    return df[out_cols].reset_index(drop=True)


def _convert_to_title(df, column):
    """Convert the values of a DataFrame column to titles"""
    df[column] = df[column].str.strip().str.casefold().str.title()
    return df
