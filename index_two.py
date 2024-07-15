names = ["Misha", "Lesha", "Nikita"]
surnames = ["Perevyazko", "Perevyazko", "Zemtcov"]
ages = [23, 23, 24]

employers_info = []

employers_info.append(names)
employers_info.append(surnames)
employers_info.append(ages)

for name in names:
    for surname in surnames:
        for age in ages:
            employer = []
            employer.append(name)
            employer.append(surname)
            employer.append(age)
            employers_info.append(employer)

print(employers_info)
