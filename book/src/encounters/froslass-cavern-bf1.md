<!-- area-id: froslass-cavern-bf1 -->
### Froslass Cavern BF1

_Source: Pokemon Lazarus Encounters PDF_

| Pokémon | Grass (Day) | ☑ |
| --- | --- | --- |
| <a href="../pokemon-lookup.html?q=alolan-vulpix">Alolan Vulpix</a> | 20% | <input type="checkbox" class="encounter-check" data-area="froslass-cavern-bf1" data-species="alolan-vulpix" /> |
| <a href="../pokemon-lookup.html?q=amaura">Amaura</a> | 2% | <input type="checkbox" class="encounter-check" data-area="froslass-cavern-bf1" data-species="amaura" /> |
| <a href="../pokemon-lookup.html?q=bronzor">Bronzor</a> | 20% | <input type="checkbox" class="encounter-check" data-area="froslass-cavern-bf1" data-species="bronzor" /> |
| <a href="../pokemon-lookup.html?q=onix">Onix</a> | 10% | <input type="checkbox" class="encounter-check" data-area="froslass-cavern-bf1" data-species="onix" /> |
| <a href="../pokemon-lookup.html?q=snorunt">Snorunt</a> | 30% | <input type="checkbox" class="encounter-check" data-area="froslass-cavern-bf1" data-species="snorunt" /> |
| <a href="../pokemon-lookup.html?q=spheal">Spheal</a> | 10% | <input type="checkbox" class="encounter-check" data-area="froslass-cavern-bf1" data-species="spheal" /> |
| <a href="../pokemon-lookup.html?q=swinub">Swinub</a> | 8% | <input type="checkbox" class="encounter-check" data-area="froslass-cavern-bf1" data-species="swinub" /> |

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

