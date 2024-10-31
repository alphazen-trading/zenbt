# from benchmarks import bench


def dev():
    from bench import bench
    # from .dev import dev
    # from .side_by_side import sbs

    # sbs()
    # dev()
    bench()


def ma_cross():
    from .ma_cross import ma_cross

    ma_cross()


# dev()
