using System;
using System.Collections.Generic;
using System.ComponentModel.DataAnnotations;
using System.Linq;

public static class Dominoes
{
    public static bool CanChain(IEnumerable<(int, int)> dominoes) => IsChain(dominoes.ToList(), (0,0));

    public static bool IsChain(List<(int, int)> dominoes, (int first, int last) state)
    {
        if (dominoes.Count == 0 && state.first == state.last) return true;

        for (int i = 0; i < dominoes.Count; i++) {
            var (a, b) = dominoes[i];
            if (state.last == 0) state = (a, b);
            else if (state.last == a) state.last = b;
            else if (state.last == b) state.last = a;
            else continue;

            var dominoesCopy = new List<(int, int)>(dominoes);
            dominoesCopy.RemoveAt(i);
            if (IsChain(dominoesCopy, state)) return true;
        }

        return false;
    }
}
