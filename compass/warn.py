"""Custom Warnings for COMPASS"""

import logging


logger = logging.getLogger("compass")


class COMPASSWarning(UserWarning):
    """Generic COMPASS Warning"""

    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)
        if args:
            logger.warning(str(args[0]), stacklevel=2)
