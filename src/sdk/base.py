from zenbt.rs import (
    OrderType,
    Side,
    Strategy,
    PySharedState,
    Action,
    Order,
)
from typing import Optional


class BaseStrategy(Strategy):
    index: int
    state: PySharedState

    def create_market_order(
        self,
        index,
        client_order_id: str,
        side: Side,
        size: float,
        sl: Optional[float] = None,
        tp: Optional[float] = None,
    ) -> Order:
        return Order(
            index=index,
            client_order_id=client_order_id,
            order_type=OrderType.Market,
            side=side,
            size=size,
            price=None,
            sl=sl,
            tp=tp,
        )

    # def active_long_positions(self) -> list[Position]:
    #     positions = []
    #     for pos in self.state.active_positions:
    #         if pos.side == Side.Long:
    #             positions.append(pos)
    #     return positions

    # def active_short_positions(self) -> list[Position]:
    #     positions = []
    #     for pos in self.state.active_positions:
    #         if pos.side == Side.Short:
    #             positions.append(pos)
    #     return positions

    # def has_position(self) -> bool:
    #     return len(self.state.active_positions) > 0


DefaultAction = Action(orders={}, position=None, close_all_positions=False)
