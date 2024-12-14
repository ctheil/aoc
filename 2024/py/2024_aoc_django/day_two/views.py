from django.http import HttpResponse
from . import problems


def index(request):
    return HttpResponse(
        "Hello, world. Welcome to Day Two of Advent of Code featuring Django"
    )


def part_one(request):
    ans = problems.part_one()
    return HttpResponse("AOC\n    Day Two\n        Part One: " + ans)


def part_two(request):
    ans = problems.part_two()
    return HttpResponse("AOC\n    Day Two\n        Part Two: " + ans)


def day_three(request):
    part = request.GET.get("part", "default")
    print(part)
    problems.day_three(part)
    return HttpResponse("AOC Day Three:Part " + part)
