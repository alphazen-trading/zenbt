# def run(i):
#     # print("We are in run:", i)

#     return 2


class Strategy:
    # def test(self, equity=1):
    #     # print("We are in run:", i)
    #     print(equity)

    #     return equity
    def major(self, equity=1, *args, **kwargs):
        # print()
        # print(args)
        # print(kwargs)
        # print(equity)
        return 4
        # print(equity)
        # # print(args)
        # print("We are in run:", equity)
        # return 3

    def __enter__(self, *args, **kwargs):
        print("We are in run:", 1)
