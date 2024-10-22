import bt
import pandas as pd


class WeighTarget(bt.Algo):
    """
    Sets target weights based on a target weight DataFrame.

    Args:
        * target_weights (DataFrame): DataFrame containing the target weights

    Sets:
        * weights

    """

    def __init__(self, target_weights):
        self.tw = target_weights

    def __call__(self, target):
        # get target weights on date target.now
        if target.now in self.tw.index:
            w = self.tw.loc[target.now]

            # save in temp - this will be used by the weighing algo
            # also dropping any na's just in case they pop up
            target.temp["weights"] = w.dropna()

        # return True because we want to keep on moving down the stack
        return True


class BT:
    def __init__(self, df):
        # ## download some data & calc SMAs

        sma50 = df.rolling(10).mean()
        sma200 = df.rolling(50).mean()

        ## now we need to calculate our target weight DataFrame
        # first we will copy the sma200 DataFrame since our weights will have the same strucutre
        tw = sma200.copy()
        # set appropriate target weights
        tw[sma50 > sma200] = 1.0
        tw[sma50 <= sma200] = -1.0
        # here we will set the weight to 0 - this is because the sma200 needs 200 data points before
        # calculating its first point. Therefore, it will start with a bunch of nulls (NaNs).
        tw[sma200.isnull()] = 0.0

        self.ma_cross = bt.Strategy("ma_cross", [WeighTarget(tw), bt.algos.Rebalance()])
        self.bt = bt
        self.data = df

    def backtest(self):
        t = self.bt.Backtest(self.ma_cross, self.data, 1000000)
        res = bt.run(t)
        return res, t
