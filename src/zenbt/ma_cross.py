from zenbt.sdk.base import BaseStrategy
from zenbt.rs import (
    Side,
    Action,
)


class MaCross(BaseStrategy):
    def on_candle(self, state=None, **kwargs) -> Action:  # type: ignore
        cross_below = self.data["cross_below"][self.index]
        cross_above = self.data["cross_above"][self.index]

        # Check for bullish cross over
        if cross_above:
            order = self.create_market_order(
                self.index,
                client_order_id="Long",
                side=Side.Long,
                size=self.default_size,
            )
            self.action.orders = {order.client_order_id: order}
            self.action.close_all_positions = True

        # Check for bearish crossover
        if cross_below:
            order = self.create_market_order(
                self.index,
                client_order_id="Short",
                side=Side.Short,
                size=self.default_size,
            )
            self.action.orders = {order.client_order_id: order}
            self.action.close_all_positions = True

        return self.action