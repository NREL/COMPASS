"""COMPASS CLI progress bars"""

import logging

from rich.progress import Progress, BarColumn, TextColumn, SpinnerColumn

from compass.exceptions import COMPASSValueError, COMPASSNotInitializedError


logger = logging.getLogger(__name__)


class COMPASSProgressBars:
    """COMPASS progress bar configurations"""

    def __init__(self, console=None):
        """

        Parameters
        ----------
        console : rich.Console, optional
            Optional Console instance. Default is an internal Console
            instance writing to stdout. By default, ``None``.
        """
        self.console = console
        self.main = Progress(
            SpinnerColumn(style="dim"),
            TextColumn("[cyan]{task.description}"),
            BarColumn(complete_style="yellow", finished_style="yellow"),
            TextColumn("[progress.percentage]{task.percentage:>3.0f}%"),
            console=console,
        )

        self.sub = Progress(
            TextColumn("    "),
            SpinnerColumn(style="dim"),
            TextColumn("[magenta]{task.description}"),
            BarColumn(
                bar_width=30, complete_style="yellow", finished_style="yellow"
            ),
            TextColumn("{task.percentage:>3.0f}%"),
            console=console,
        )
        self._main_task = None

    def create_main_task(self, num_jurisdictions):
        """Set up main task to track number of jurisdictions processed

        Parameters
        ----------
        num_jurisdictions : int
            Number of jurisdictions that are being processed.

        Raises
        ------
        COMPASSValueError
            If the main task has already been set up.
        """
        if self._main_task is not None:
            msg = "Main task has already been set!"
            raise COMPASSValueError(msg)

        logger.trace(
            "Starting main progress bar with %d jurisdictions",
            num_jurisdictions,
        )
        self._main_task = self.main.add_task(
            f"[bold cyan]Searching {num_jurisdictions} Jurisdictions",
            total=num_jurisdictions,
        )

    def progress_main_task(self):
        """Advance the main task one step

        In other words, mark one jurisdiction as complete.

        Raises
        ------
        COMPASSNotInitializedError
            If the main task has not been set up (i.e.
            `create_main_task` has not been called).
        """
        if self._main_task is None:
            msg = (
                "Main task has not been created! Call the "
                "`create_main_task` method first"
            )
            raise COMPASSNotInitializedError(msg)

        self.main.update(self._main_task, advance=1)
