from django.urls import path
from . import views

urlpatterns = [
    path("", views.index, name="index"),
    path("ptone", views.part_one, name="Part One"),
    path("pttwo", views.part_two, name="Part Two"),
    path("three", views.day_three, name="Day Three"),
]
