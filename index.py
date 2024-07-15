from dataclasses import dataclass


@dataclass
class Card:
    rank: str
    suit: str

card = Card("Q", "hearts")

print(card.rank)


add_one = lambda x: x+1
print(add_one(3))


numbers = [1, 2, 3, 4]
times_two = map(lambda x: x*2, numbers)
print(list(times_two))


for number in numbers:
    doubled_number = number * 2
    print(doubled_number)



squares_arr = [x**2 for x in range(20)]
print(squares_arr)


dict1 = {'a' : 1, 'b' : 2 }
dict2 = {'a' : 1, 'c' : 3 }

merged_dict = dict1 | dict2

print(merged_dict)

arr = [1, 2, 3, 4, 5]
first_two_arr = arr[0:2]
print(first_two_arr)


for i in arr:
    if i != 3:
        print(i)
    else: 
        break


#! import jmespath
#persons = {
#     "persons": [
#         {"name" : "eric", "age" : 38},
#         {"name" : "mick", "age" : 21},
#         {"name" : "flick", "age" : 18},
#     ]
# }

# jmespath.search('persons[*].age', persons)
    

new_string = 'abcdefg'
reversed_string = new_string[::-1]
print(reversed_string)


int_list = [-3,-2,-2,-1,-1,0,0,0,1,1,1,2,2,3,4,5,5,6,7,8,8,9,10]
special_list = set(int_list)
print(special_list)
print(min(special_list))
print(max(special_list))
