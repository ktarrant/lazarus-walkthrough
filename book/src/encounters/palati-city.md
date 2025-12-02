<!-- area-id: palati-city -->
### Palati City

_Source: Pokemon Lazarus Encounters PDF_

| Pokémon | Grass (Day) | Grass (Night) | Surfing | Old Rod | Good Rod | Super Rod | ☑ |
| --- | --- | --- | --- | --- | --- | --- | --- |
| <a href="../pokemon-lookup.html?q=archen">Archen</a> | 10% | 10% | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="palati-city" data-species="archen" /> |
| <a href="../pokemon-lookup.html?q=baltoy">Baltoy</a> | 20% | 20% | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="palati-city" data-species="baltoy" /> |
| <a href="../pokemon-lookup.html?q=girafarig">Girafarig</a> | 10% | 10% | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="palati-city" data-species="girafarig" /> |
| <a href="../pokemon-lookup.html?q=gurdurr">Gurdurr</a> | 4% | — | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="palati-city" data-species="gurdurr" /> |
| <a href="../pokemon-lookup.html?q=klawf">Klawf</a> | 20% | 20% | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="palati-city" data-species="klawf" /> |
| <a href="../pokemon-lookup.html?q=lycanroc-day">Lycanroc Day</a> | 2% | — | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="palati-city" data-species="lycanroc-day" /> |
| <a href="../pokemon-lookup.html?q=pawmo">Pawmo</a> | 10% | — | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="palati-city" data-species="pawmo" /> |
| <a href="../pokemon-lookup.html?q=quagsire">Quagsire</a> | 10% | — | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="palati-city" data-species="quagsire" /> |
| <a href="../pokemon-lookup.html?q=timburr">Timburr</a> | 4% | — | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="palati-city" data-species="timburr" /> |
| <a href="../pokemon-lookup.html?q=toedscool">Toedscool</a> | 10% | — | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="palati-city" data-species="toedscool" /> |
| <a href="../pokemon-lookup.html?q=clodsire">Clodsire</a> | — | 10% | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="palati-city" data-species="clodsire" /> |
| <a href="../pokemon-lookup.html?q=grafaiai">Grafaiai</a> | — | 10% | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="palati-city" data-species="grafaiai" /> |
| <a href="../pokemon-lookup.html?q=lycanroc-night">Lycanroc Night</a> | — | 2% | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="palati-city" data-species="lycanroc-night" /> |
| <a href="../pokemon-lookup.html?q=munna">Munna</a> | — | 10% | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="palati-city" data-species="munna" /> |
| <a href="../pokemon-lookup.html?q=toedscruel">Toedscruel</a> | — | 8% | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="palati-city" data-species="toedscruel" /> |
| <a href="../pokemon-lookup.html?q=tentacool">Tentacool</a> | — | — | 60% | 30% | — | — | <input type="checkbox" class="encounter-check" data-area="palati-city" data-species="tentacool" /> |
| <a href="../pokemon-lookup.html?q=tirtouga">Tirtouga</a> | — | — | 10% | — | 60% | 15% | <input type="checkbox" class="encounter-check" data-area="palati-city" data-species="tirtouga" /> |
| <a href="../pokemon-lookup.html?q=wailmer">Wailmer</a> | — | — | 30% | — | — | 40% | <input type="checkbox" class="encounter-check" data-area="palati-city" data-species="wailmer" /> |
| <a href="../pokemon-lookup.html?q=magikarp">Magikarp</a> | — | — | — | 70% | — | — | <input type="checkbox" class="encounter-check" data-area="palati-city" data-species="magikarp" /> |
| <a href="../pokemon-lookup.html?q=clamperl">Clamperl</a> | — | — | — | — | 20% | 40% | <input type="checkbox" class="encounter-check" data-area="palati-city" data-species="clamperl" /> |
| <a href="../pokemon-lookup.html?q=shellder">Shellder</a> | — | — | — | — | 20% | — | <input type="checkbox" class="encounter-check" data-area="palati-city" data-species="shellder" /> |
| <a href="../pokemon-lookup.html?q=dhelmise">Dhelmise</a> | — | — | — | — | — | 5% | <input type="checkbox" class="encounter-check" data-area="palati-city" data-species="dhelmise" /> |

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

