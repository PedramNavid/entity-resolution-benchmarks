import entity


def test_ngrams():
    res = entity.tokenize("Pédram Navid, P]]dram", n=3)

    assert res == {"péd", "édr", "ram", "nav", "avi", "vid", "pdr", "dra", "amn", "mna"}


def test_levenshtein():
    res = entity.score_lev_distance("Pédram Navid", "Pedram Novis")
    assert res == 0.75
    res = entity.score_lev_distance("Pedram", "Bob")
    assert res == 0
    res = entity.score_lev_distance("Pedram", "Pedram")
    assert res == 1
