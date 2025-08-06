"""COMPASS main CLI entrypoint"""

import click

from compass import __version__
from compass._cli.process import process


@click.group()
@click.version_option(version=__version__)
@click.pass_context
def main(ctx):
    """Ordinance command line interface"""
    ctx.ensure_object(dict)


main.add_command(process)


if __name__ == "__main__":
    main(obj={})
