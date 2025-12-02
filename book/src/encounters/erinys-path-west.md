<!-- area-id: erinys-path-west -->
### Erinys Path (West)

_Source: Pokemon Lazarus Encounters PDF_

| Pokémon | Grass (Day) | Grass (Night) | ☑ |
| --- | --- | --- | --- |
| <a href="../pokemon-lookup.html?q=baltoy">Baltoy</a> | 10% | 20% | <input type="checkbox" class="encounter-check" data-area="erinys-path-west" data-species="baltoy" /> |
| <a href="../pokemon-lookup.html?q=komala">Komala</a> | 10% | — | <input type="checkbox" class="encounter-check" data-area="erinys-path-west" data-species="komala" /> |
| <a href="../pokemon-lookup.html?q=minior-blue-core">Minior Blue Core</a> | 4% | — | <input type="checkbox" class="encounter-check" data-area="erinys-path-west" data-species="minior-blue-core" /> |
| <a href="../pokemon-lookup.html?q=minior-indigo-core">Minior Indigo Core</a> | 4% | — | <input type="checkbox" class="encounter-check" data-area="erinys-path-west" data-species="minior-indigo-core" /> |
| <a href="../pokemon-lookup.html?q=omanyte">Omanyte</a> | 10% | 10% | <input type="checkbox" class="encounter-check" data-area="erinys-path-west" data-species="omanyte" /> |
| <a href="../pokemon-lookup.html?q=sandile">Sandile</a> | 20% | — | <input type="checkbox" class="encounter-check" data-area="erinys-path-west" data-species="sandile" /> |
| <a href="../pokemon-lookup.html?q=solosis">Solosis</a> | 20% | — | <input type="checkbox" class="encounter-check" data-area="erinys-path-west" data-species="solosis" /> |
| <a href="../pokemon-lookup.html?q=togepi">Togepi</a> | 2% | 2% | <input type="checkbox" class="encounter-check" data-area="erinys-path-west" data-species="togepi" /> |
| <a href="../pokemon-lookup.html?q=zigzagoon">Zigzagoon</a> | 10% | — | <input type="checkbox" class="encounter-check" data-area="erinys-path-west" data-species="zigzagoon" /> |
| <a href="../pokemon-lookup.html?q=zorua">Zorua</a> | 10% | — | <input type="checkbox" class="encounter-check" data-area="erinys-path-west" data-species="zorua" /> |
| <a href="../pokemon-lookup.html?q=galarian-zigzagoon">Galarian Zigzagoon</a> | — | 10% | <input type="checkbox" class="encounter-check" data-area="erinys-path-west" data-species="galarian-zigzagoon" /> |
| <a href="../pokemon-lookup.html?q=gothita">Gothita</a> | — | 20% | <input type="checkbox" class="encounter-check" data-area="erinys-path-west" data-species="gothita" /> |
| <a href="../pokemon-lookup.html?q=minior-orange-core">Minior Orange Core</a> | — | 4% | <input type="checkbox" class="encounter-check" data-area="erinys-path-west" data-species="minior-orange-core" /> |
| <a href="../pokemon-lookup.html?q=minior-red-core">Minior Red Core</a> | — | 4% | <input type="checkbox" class="encounter-check" data-area="erinys-path-west" data-species="minior-red-core" /> |
| <a href="../pokemon-lookup.html?q=stantler">Stantler</a> | — | 20% | <input type="checkbox" class="encounter-check" data-area="erinys-path-west" data-species="stantler" /> |
| <a href="../pokemon-lookup.html?q=zorua-hisuian">Zorua Hisuian</a> | — | 10% | <input type="checkbox" class="encounter-check" data-area="erinys-path-west" data-species="zorua-hisuian" /> |

<script>
(function() {
  const STORAGE_KEY = 'lazarusEncounterChecks';
  function loadState() {
    try {
      return JSON.parse(localStorage.getItem(STORAGE_KEY) || '{}');
    } catch (_) {
      return {};
    }
  }
  function saveState(state) {
    try {
      localStorage.setItem(STORAGE_KEY, JSON.stringify(state));
    } catch (_) {}
  }
  const state = loadState();
  const checkboxes = document.querySelectorAll('.encounter-check');
  checkboxes.forEach(cb => {
    const key = `${cb.dataset.area}|${cb.dataset.species}`;
    cb.checked = !!state[key];
    cb.addEventListener('change', () => {
      if (cb.checked) {
        state[key] = true;
      } else {
        delete state[key];
      }
      saveState(state);
    });
  });
})();
</script>

