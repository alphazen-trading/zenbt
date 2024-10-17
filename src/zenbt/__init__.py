# from benchmarks import bench
from bench import bench


def dev():
    from .dev import dev
    # from .side_by_side import sbs
    # bench()

    # sbs()
    dev()
    # bench()


def ma_cross():
    from .ma_cross import ma_cross

    ma_cross()


# dev()
