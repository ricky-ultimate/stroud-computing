class Round:
    def __init__(self, num):
        self.num = num

    def round10(self):
        if self.num >= 0:
            tens_value = self.num % 10
            if tens_value >= 5:
                print(self.num + (10 - tens_value))
            else:
                print(self.num - tens_value)
        else:
            tens_value = abs(self.num) % 10
            if tens_value >= 5:
                print(self.num - (10 - tens_value))
            else:
                print(self.num + tens_value)

    def round100(self):
        if self.num >= 0:
            hundreds_value = self.num % 100
            if hundreds_value >= 50:
                print(self.num + (100 - hundreds_value))
            else:
                print(self.num - hundreds_value)
        else:
            hundreds_value = abs(self.num) % 100
            if hundreds_value >= 50:
                print(self.num - (100 - hundreds_value))
            else:
                print(self.num + hundreds_value)

    def round1000(self):
        if self.num >= 0:
            thousands_value = self.num % 1000
            if thousands_value >= 500:
                print(self.num + (1000 - thousands_value))
            else:
                print(self.num - thousands_value)
        else:
            thousands_value = abs(self.num) % 1000
            if thousands_value >= 500:
                print(self.num - (1000 - thousands_value))
            else:
                print(self.num + thousands_value)


numbers = [1846, -638, 445]
for number in numbers:
    rounded = Round(number)
    rounded.round10()
    rounded.round100()
    rounded.round1000()
    print("\n")
    