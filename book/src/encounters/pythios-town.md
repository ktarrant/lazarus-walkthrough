<!-- area-id: pythios-town -->
### Pythios Town

_Source: Pokemon Lazarus Encounters PDF_

| Pokémon | Grass (Day) | Grass (Night) | Surfing | Old Rod | Good Rod | Super Rod | ☑ |
| --- | --- | --- | --- | --- | --- | --- | --- |
| <a href="../pokemon-lookup.html?q=doduo">Doduo</a> | 10% | — | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="pythios-town" data-species="doduo" /> |
| <a href="../pokemon-lookup.html?q=drowzee">Drowzee</a> | 10% | 10% | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="pythios-town" data-species="drowzee" /> |
| <a href="../pokemon-lookup.html?q=dwebble">Dwebble</a> | 20% | 20% | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="pythios-town" data-species="dwebble" /> |
| <a href="../pokemon-lookup.html?q=flittle">Flittle</a> | 10% | 10% | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="pythios-town" data-species="flittle" /> |
| <a href="../pokemon-lookup.html?q=honedge">Honedge</a> | 2% | 2% | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="pythios-town" data-species="honedge" /> |
| <a href="../pokemon-lookup.html?q=meowth">Meowth</a> | 10% | — | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="pythios-town" data-species="meowth" /> |
| <a href="../pokemon-lookup.html?q=mienfoo">Mienfoo</a> | 10% | 10% | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="pythios-town" data-species="mienfoo" /> |
| <a href="../pokemon-lookup.html?q=rufflet">Rufflet</a> | 20% | — | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="pythios-town" data-species="rufflet" /> |
| <a href="../pokemon-lookup.html?q=wingull">Wingull</a> | 8% | 8% | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="pythios-town" data-species="wingull" /> |
| <a href="../pokemon-lookup.html?q=alolan-meowth">Alolan Meowth</a> | — | 10% | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="pythios-town" data-species="alolan-meowth" /> |
| <a href="../pokemon-lookup.html?q=hoothoot">Hoothoot</a> | — | 10% | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="pythios-town" data-species="hoothoot" /> |
| <a href="../pokemon-lookup.html?q=vullaby">Vullaby</a> | — | 20% | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="pythios-town" data-species="vullaby" /> |
| <a href="../pokemon-lookup.html?q=finizen">Finizen</a> | — | — | 10% | — | 20% | — | <input type="checkbox" class="encounter-check" data-area="pythios-town" data-species="finizen" /> |
| <a href="../pokemon-lookup.html?q=seel">Seel</a> | — | — | 30% | — | — | — | <input type="checkbox" class="encounter-check" data-area="pythios-town" data-species="seel" /> |
| <a href="../pokemon-lookup.html?q=spheal">Spheal</a> | — | — | 60% | — | 20% | — | <input type="checkbox" class="encounter-check" data-area="pythios-town" data-species="spheal" /> |
| <a href="../pokemon-lookup.html?q=corphish">Corphish</a> | — | — | — | 30% | — | — | <input type="checkbox" class="encounter-check" data-area="pythios-town" data-species="corphish" /> |
| <a href="../pokemon-lookup.html?q=magikarp">Magikarp</a> | — | — | — | 70% | — | — | <input type="checkbox" class="encounter-check" data-area="pythios-town" data-species="magikarp" /> |
| <a href="../pokemon-lookup.html?q=poliwag">Poliwag</a> | — | — | — | — | 60% | — | <input type="checkbox" class="encounter-check" data-area="pythios-town" data-species="poliwag" /> |
| <a href="../pokemon-lookup.html?q=finneon">Finneon</a> | — | — | — | — | — | 40% | <input type="checkbox" class="encounter-check" data-area="pythios-town" data-species="finneon" /> |
| <a href="../pokemon-lookup.html?q=remoraid">Remoraid</a> | — | — | — | — | — | 40% | <input type="checkbox" class="encounter-check" data-area="pythios-town" data-species="remoraid" /> |

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

