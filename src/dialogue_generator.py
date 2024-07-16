# Copyright (c) 2024 Cutieguwu | Olivia Brooks
#
# -*- coding:utf-8 -*-
# @Title: Dialogue Generator
# @Author: Cutieguwu | Olivia Brooks
# @Description: Dialogue parser and generator because it's easier to do the modifications in python..
#
# @Script: dialogue_generator.py
# @Date Created: 08 Jul, 2024
# @Last Modified: 09 Jul, 2024
# @Last Modified by: Cutieguwu | Olivia Brooks
# ----------------------------------------------------------

from enum import Enum

Characters = Enum(["RedRidingHood", "BadWolf"])
TextReferencce = Enum(["Personal", "PosessiveAdjective", "PosessivePronoun", "Reflexive"])
TextCase = Enum(["Upper", "Lower"])

def pronoun(target, reference, case):
    pass

DIALOGUE = f"Once upon a time, there lived a little girl. {pronoun(Characters.RedRidingHood, TextReferencce.Personal, TextCase.Upper)} had a red hood that {pronoun(Characters.RedRidingHood, TextReferencce.PosessiveAdjective, TextCase.Lower)} grandmother made."