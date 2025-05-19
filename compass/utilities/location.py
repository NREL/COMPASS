"""COMPASS Ordinance location specification utilities"""


class Jurisdiction:
    """Class representing a jurisdiction"""

    def __init__(self, name, state, code=None, is_parish=False):
        """

        Parameters
        ----------
        name : str
            Name of the jurisdiction.
        state : str
            State containing the jurisdiction.
        code : int or str, optional
            Optional jurisdiction code (typically FIPS or similar).
            By default, ``None``.
        is_parish : bool, optional
            Flag indicating whether or not this jurisdiction is
            classified as a parish. By default, ``False``.
        """
        self.name = name
        self.state = state
        self.code = code
        self.is_parish = is_parish

    @property
    def full_name(self):
        """str: Full name in format '{name} County, {state}'"""
        loc_id = "Parish" if self.is_parish else "County"
        return f"{self.name} {loc_id}, {self.state}"

    def __repr__(self):
        return (
            f"Jurisdiction({self.name}, {self.state}, "
            f"is_parish={self.is_parish})"
        )

    def __str__(self):
        return self.full_name

    def __eq__(self, other):
        if isinstance(other, self.__class__):
            return (
                self.name.casefold() == other.name.casefold()
                and self.state.casefold() == other.state.casefold()
                and self.is_parish == other.is_parish
            )
        if isinstance(other, str):
            return (
                self.full_name.casefold() == other.casefold()
                or f"{self.name}, {self.state}".casefold() == other.casefold()
            )
        return False

    def __hash__(self):
        return hash(self.full_name.casefold())
