from zenbt.sdk import BaseStrategy
from zenbt.zbt import Action, PySharedState, Side, Position
from datetime import datetime


class Strategy(BaseStrategy):
    last_pos_id = None

    def on_candle(self, state: PySharedState = None, **kwargs) -> Action:
        print("On candle")
        entry = self.data["entry"][self.index]

        if state.active_position is None:
            if entry > 0:
                sl = self.data["sl"][self.index]
                # print(f"We have an entry {entry} with sl {sl}")
                side = Side.Long if sl < entry else Side.Short
                dt = datetime.fromtimestamp(self.data["time"][self.index] / 1000)
                print(dt)

                if side == Side.Long:
                    tp = entry + (entry - sl)
                else:
                    tp = entry - (sl - entry)

                order = self.create_limit_order(
                    self.index,
                    client_order_id="order",
                    side=side,
                    size=self.default_size,
                    price=entry,
                    sl=sl,
                    tp=tp,
                )
                # self.action.orders = {order.client_order_id: order}
                # self.action.close_all_positions = True
                return Action(
                    orders={order.client_order_id: order},
                    close_all_positions=True,
                )
        else:
            pos: Position = state.active_position

        return self.action
