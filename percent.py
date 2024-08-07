print("\n")
started_summ = int(input("Введите начальную сумму: "))
percent = float(input("Введите процент вклада: "))
capitalization = 0
mounths = int(input("Введите срок вклада в месяцах: "))
added_sum = int(input("Введите на какую сумму будете ежемесячно пополнять вклад: "))
end_summ = started_summ
print("\n")

def count(started_summ,percent,capitalization,mounths,added_sum,end_summ):
    mounth = 0
    full_period_percent = 0
    while mounth < mounths:
        capitalization = (started_summ * (percent / 100)) / 12
        end_summ = round(started_summ + capitalization, 0)
        full_period_percent += capitalization
        capitalization = 0
        started_summ = end_summ + added_sum
        mounth += 1
        print(f"В конце {mounth} месяца:" ,started_summ)
        print(f"Общий % = {round(full_period_percent, 2)} руб. \n")
    return end_summ

count(started_summ, percent, capitalization, mounths, added_sum, end_summ)