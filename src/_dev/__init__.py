from .debugging import import_pickl
from .debugging import test_pickl


def dev():
    from .mt5 import dev as _dev
    # from .dev import dev as _dev
    # from .indicator import dev as _dev

    _dev()
