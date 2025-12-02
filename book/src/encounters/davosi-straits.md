<!-- area-id: davosi-straits -->
### Davosi Straits

_Source: Pokemon Lazarus Encounters PDF_

| Pokémon | Surfing | Old Rod | Good Rod | Super Rod | ☑ |
| --- | --- | --- | --- | --- | --- |
| <a href="../pokemon-lookup.html?q=dhelmise">Dhelmise</a> | 5% | — | — | — | <input type="checkbox" class="encounter-check" data-area="davosi-straits" data-species="dhelmise" /> |
| <a href="../pokemon-lookup.html?q=dragonair">Dragonair</a> | 5% | — | — | — | <input type="checkbox" class="encounter-check" data-area="davosi-straits" data-species="dragonair" /> |
| <a href="../pokemon-lookup.html?q=finizen">Finizen</a> | 60% | — | — | — | <input type="checkbox" class="encounter-check" data-area="davosi-straits" data-species="finizen" /> |
| <a href="../pokemon-lookup.html?q=swanna">Swanna</a> | 30% | — | — | — | <input type="checkbox" class="encounter-check" data-area="davosi-straits" data-species="swanna" /> |
| <a href="../pokemon-lookup.html?q=magikarp">Magikarp</a> | — | 70% | — | — | <input type="checkbox" class="encounter-check" data-area="davosi-straits" data-species="magikarp" /> |
| <a href="../pokemon-lookup.html?q=remoraid">Remoraid</a> | — | 30% | 20% | — | <input type="checkbox" class="encounter-check" data-area="davosi-straits" data-species="remoraid" /> |
| <a href="../pokemon-lookup.html?q=finneon">Finneon</a> | — | — | 20% | — | <input type="checkbox" class="encounter-check" data-area="davosi-straits" data-species="finneon" /> |
| <a href="../pokemon-lookup.html?q=shellder">Shellder</a> | — | — | 60% | — | <input type="checkbox" class="encounter-check" data-area="davosi-straits" data-species="shellder" /> |
| <a href="../pokemon-lookup.html?q=cloyster">Cloyster</a> | — | — | — | 15% | <input type="checkbox" class="encounter-check" data-area="davosi-straits" data-species="cloyster" /> |
| <a href="../pokemon-lookup.html?q=golduck">Golduck</a> | — | — | — | 40% | <input type="checkbox" class="encounter-check" data-area="davosi-straits" data-species="golduck" /> |
| <a href="../pokemon-lookup.html?q=gorebyss">Gorebyss</a> | — | — | — | 5% | <input type="checkbox" class="encounter-check" data-area="davosi-straits" data-species="gorebyss" /> |
| <a href="../pokemon-lookup.html?q=octillery">Octillery</a> | — | — | — | 40% | <input type="checkbox" class="encounter-check" data-area="davosi-straits" data-species="octillery" /> |

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

