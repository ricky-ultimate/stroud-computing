class Round:
    def __init__(self, num):
        self.num = num

    def round10(self):
        tens_value = self.num % 10
        if tens_value >= 5:
            print(self.num + (10 - tens_value))
        else:
            print(self.num - tens_value)

    def round100(self):
        hundreds_value = self.num % 100
        if hundreds_value >= 50:
            print(self.num + (100 - hundreds_value))
        else:
            print(self.num - hundreds_value)

    def round1000(self):
        thousands_value = self.num % 1000
        if thousands_value >= 500:
            print(self.num + (1000 - thousands_value))
        else:
            print(self.num - thousands_value)


num1 = Round(1846)
num1.round10()
num1.round100()
num1.round1000()
