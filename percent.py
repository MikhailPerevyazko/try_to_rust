started_summ = 360000
percent = 14
capitalization = 0
mounths = 6
added_sum = 40000
end_summ = started_summ

def count(started_summ,percent,capitalization,mounths,added_sum,end_summ):
    mounth = 0
    while mounth < mounths:
        capitalization = (started_summ*(percent/100))/12
        end_summ = round(started_summ + capitalization, 0)
        capitalization = 0
        started_summ = end_summ + added_sum
        mounth += 1
        print(f"В конце {mounth} месяца:" ,end_summ)

count(started_summ, percent, capitalization, mounths, added_sum, end_summ)
