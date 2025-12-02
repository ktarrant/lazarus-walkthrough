<!-- area-id: acrisia-city -->
### Acrisia City

_Source: Pokemon Lazarus Encounters PDF_

| Pokémon | Grass (Day) | Grass (Night) | Surfing | Old Rod | Good Rod | Super Rod | ☑ |
| --- | --- | --- | --- | --- | --- | --- | --- |
| <a href="../pokemon-lookup.html?q=ducklett">Ducklett</a> | 10% | — | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="acrisia-city" data-species="ducklett" /> |
| <a href="../pokemon-lookup.html?q=grimer">Grimer</a> | 5% | — | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="acrisia-city" data-species="grimer" /> |
| <a href="../pokemon-lookup.html?q=panpour">Panpour</a> | 5% | 5% | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="acrisia-city" data-species="panpour" /> |
| <a href="../pokemon-lookup.html?q=pansage">Pansage</a> | 5% | 5% | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="acrisia-city" data-species="pansage" /> |
| <a href="../pokemon-lookup.html?q=pansear">Pansear</a> | 5% | 5% | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="acrisia-city" data-species="pansear" /> |
| <a href="../pokemon-lookup.html?q=phanpy">Phanpy</a> | 10% | 10% | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="acrisia-city" data-species="phanpy" /> |
| <a href="../pokemon-lookup.html?q=pikipek">Pikipek</a> | 20% | — | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="acrisia-city" data-species="pikipek" /> |
| <a href="../pokemon-lookup.html?q=poliwag">Poliwag</a> | 10% | 10% | — | — | 60% | — | <input type="checkbox" class="encounter-check" data-area="acrisia-city" data-species="poliwag" /> |
| <a href="../pokemon-lookup.html?q=sandshrew">Sandshrew</a> | 20% | 20% | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="acrisia-city" data-species="sandshrew" /> |
| <a href="../pokemon-lookup.html?q=shroomish">Shroomish</a> | 10% | 10% | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="acrisia-city" data-species="shroomish" /> |
| <a href="../pokemon-lookup.html?q=alolan-grimer">Alolan Grimer</a> | — | 5% | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="acrisia-city" data-species="alolan-grimer" /> |
| <a href="../pokemon-lookup.html?q=hoothoot">Hoothoot</a> | — | 20% | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="acrisia-city" data-species="hoothoot" /> |
| <a href="../pokemon-lookup.html?q=munna">Munna</a> | — | 10% | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="acrisia-city" data-species="munna" /> |
| <a href="../pokemon-lookup.html?q=finneon">Finneon</a> | — | — | 10% | — | — | 40% | <input type="checkbox" class="encounter-check" data-area="acrisia-city" data-species="finneon" /> |
| <a href="../pokemon-lookup.html?q=remoraid">Remoraid</a> | — | — | 30% | — | — | 40% | <input type="checkbox" class="encounter-check" data-area="acrisia-city" data-species="remoraid" /> |
| <a href="../pokemon-lookup.html?q=wimpod">Wimpod</a> | — | — | 60% | 30% | — | — | <input type="checkbox" class="encounter-check" data-area="acrisia-city" data-species="wimpod" /> |
| <a href="../pokemon-lookup.html?q=magikarp">Magikarp</a> | — | — | — | 70% | — | — | <input type="checkbox" class="encounter-check" data-area="acrisia-city" data-species="magikarp" /> |
| <a href="../pokemon-lookup.html?q=wooper">Wooper</a> | — | — | — | — | 40% | — | <input type="checkbox" class="encounter-check" data-area="acrisia-city" data-species="wooper" /> |

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

