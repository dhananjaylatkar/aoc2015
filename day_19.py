"""
--- Day 19: Medicine for Rudolph ---

Rudolph the Red-Nosed Reindeer is sick! His nose isn't shining very brightly,
and he needs medicine.

Red-Nosed Reindeer biology isn't similar to regular reindeer biology; Rudolph
is going to need custom-made medicine. Unfortunately, Red-Nosed Reindeer
chemistry isn't similar to regular reindeer chemistry, either.

The North Pole is equipped with a Red-Nosed Reindeer nuclear fusion/fission
plant, capable of constructing any Red-Nosed Reindeer molecule you need. It works
by starting with some input molecule and then doing a series of replacements,
one per step, until it has the right molecule.

However, the machine has to be calibrated before it can be used. Calibration
involves determining the number of molecules that can be generated in one step
from a given starting point.

For example, imagine a simpler machine that supports only the following replacements:

H => HO
H => OH
O => HH

Given the replacements above and starting with HOH, the following molecules
could be generated:

    HOOH (via H => HO on the first H).
    HOHO (via H => HO on the second H).
    OHOH (via H => OH on the first H).
    HOOH (via H => OH on the second H).
    HHHH (via O => HH).

So, in the example above, there are 4 distinct molecules (not five, because
HOOH appears twice) after one replacement from HOH. Santa's favorite molecule,
HOHOHO, can become 7 distinct molecules (over nine replacements: six from H,
and three from O).

The machine replaces without regard for the surrounding characters. For example,
given the string H2O, the transition H => OO would result in OO2O.

Your puzzle input describes all of the possible replacements and, at the bottom,
the medicine molecule for which you need to calibrate the machine.
How many distinct molecules can be created after all the different ways you can
do one replacement on the medicine molecule?

--- Part Two ---

Now that the machine is calibrated, you're ready to begin molecule fabrication.

Molecule fabrication always begins with just a single electron, e, and applying
replacements one at a time, just like the ones during calibration.

For example, suppose you have the following replacements:

e => H
e => O
H => HO
H => OH
O => HH

If you'd like to make HOH, you start with e, and then make the following
replacements:

    e => O to get O
    O => HH to get HH
    H => OH (on the second H) to get HOH

So, you could make HOH after 3 steps. Santa's favorite molecule, HOHOHO, can be
made in 6 steps.

How long will it take to make the medicine? Given the available replacements
and the medicine molecule in your puzzle input, what is the fewest number of
steps to go from e to the medicine molecule?
"""

from pprint import pprint

with open("./src/input/day_19") as f:
    lines = f.readlines()

replacements = {}
molecule = ""
for line in lines:
    curr = [x.strip() for x in line.split(" => ")]
    if len(curr) == 2:
        if replacements.get(curr[0]):
            replacements[curr[0]].append(curr[1])
        else:
            replacements[curr[0]] = [curr[1]]
    else:
        molecule = curr[0]

# pprint(replacements)
# pprint(molecule)


def part_1(replacements, molecule):
    all_molecules = set()
    n = len(molecule)
    for i in range(n):
        if replacements.get(molecule[i]):
            for rep in replacements[molecule[i]]:
                all_molecules.add(molecule[:i] + rep + molecule[i + 1 :])

        if i < n - 1 and replacements.get(molecule[i : i + 2]):
            for rep in replacements[molecule[i : i + 2]]:
                all_molecules.add(molecule[:i] + rep + molecule[i + 2 :])

    print("D19P1:", len(all_molecules))
    return all_molecules


part_1(replacements, molecule)


def part_2(replacements, molecule):
    # This is crazyyy
    from random import shuffle

    reversed = [(v, k) for k in replacements for v in replacements[k]]
    # pprint(reversed)
    steps = 0
    target = molecule
    while target != "e":
        changed = False
        for repl, source in reversed:
            if repl in target:
                target = target.replace(repl, source, 1)
                steps += 1
                changed = True
        if not changed:
            target = molecule
            shuffle(reversed)
            steps = 0
    print("D19P2:", steps)

    ### BFS is taking too long... ###
    # from collections import deque
    # res = 0
    # q = deque()
    # s = set()
    #
    # q.append("e")
    # s.add("e")
    #
    # while q:
    #     n = len(q)
    #
    #     for i in range(n):
    #         curr = q.popleft()
    #
    #         if curr == molecule:
    #             print(res)
    #             return
    #
    #         next_molecules = part_1(replacements, curr)
    #         for nm in next_molecules:
    #             if nm not in s and len(nm) <= len(molecule):
    #                 q.append(nm)
    #                 s.add(nm)
    #
    #     res += 1


part_2(replacements, molecule)
