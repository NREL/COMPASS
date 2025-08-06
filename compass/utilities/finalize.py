"""COMPASS utilities for finalizing a run directory"""

import json
import getpass

from elm.version import __version__ as elm_version

from compass import __version__ as compass_version


def save_run_meta(
    dirs,
    tech,
    start_date,
    end_date,
    num_jurisdictions_searched,
    num_jurisdictions_found,
    total_cost,
    models,
):
    """Write out meta information about ordinance collection run"""

    try:
        username = getpass.getuser()
    except OSError:
        username = "Unknown"

    time_elapsed = end_date - start_date
    meta_data = {
        "username": username,
        "versions": {"elm": elm_version, "compass": compass_version},
        "technology": tech,
        "models": _extract_model_info_from_all_models(models),
        "time_start_utc": start_date.isoformat(),
        "time_end_utc": end_date.isoformat(),
        "total_time": time_elapsed.seconds,
        "total_time_string": str(time_elapsed),
        "num_jurisdictions_searched": num_jurisdictions_searched,
        "num_jurisdictions_found": num_jurisdictions_found,
        "cost": total_cost or None,
        "manifest": {},
    }
    manifest = {
        "LOG_DIR": dirs.logs,
        "CLEAN_FILE_DIR": dirs.clean_files,
        "JURISDICTION_DBS_DIR": dirs.jurisdiction_dbs,
        "ORDINANCE_FILES_DIR": dirs.ordinance_files,
        "USAGE_FILE": dirs.out / "usage.json",
        "JURISDICTION_FILE": dirs.out / "jurisdictions.json",
        "QUANT_DATA_FILE": dirs.out / "quantitative_ordinances.csv",
        "QUAL_DATA_FILE": dirs.out / "quantitative_ordinances.csv",
    }
    for name, file_path in manifest.items():
        if file_path.exists():
            meta_data["manifest"][name] = str(file_path.relative_to(dirs.out))
        else:
            meta_data["manifest"][name] = None

    meta_data["manifest"]["META_FILE"] = "meta.json"
    with (dirs.out / "meta.json").open("w", encoding="utf-8") as fh:
        json.dump(meta_data, fh, indent=4)

    return time_elapsed.seconds


def _extract_model_info_from_all_models(models):
    """Group model info together"""
    models_to_tasks = {}
    for task, caller_args in models.items():
        models_to_tasks.setdefault(caller_args, []).append(task)

    return [
        {
            "name": caller_args.name,
            "llm_call_kwargs": caller_args.llm_call_kwargs or None,
            "llm_service_rate_limit": caller_args.llm_service_rate_limit,
            "text_splitter_chunk_size": caller_args.text_splitter_chunk_size,
            "text_splitter_chunk_overlap": (
                caller_args.text_splitter_chunk_overlap
            ),
            "client_type": caller_args.client_type,
            "tasks": tasks,
        }
        for caller_args, tasks in models_to_tasks.items()
    ]
