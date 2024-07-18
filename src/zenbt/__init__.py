from sdk.logger import print, logger

from zenbt.rs import BBO
from zenbt.rs import add


def main() -> int:
    print(dir(BBO))
    print(add)
    # print("Hello")
    # logger.warning("Warning")
    # logger.error("Error message")
    return 0
