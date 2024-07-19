wanted_sum = int(input('Введите, какую сумму хотите получать в месяц в качестве процентов: '))
procent = int(input('Введите процентную ставку вклада: '))

def sum(a, b):
    x = ((a * 12) / (b / 100))
    rounded_x = round(x, 2)
    print('Сумма на вкладе должна быть: ', rounded_x, ' рублей.')
    return rounded_x

sum(wanted_sum, procent)


