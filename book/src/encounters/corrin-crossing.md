<!-- area-id: corrin-crossing -->
### Corrin Crossing

_Source: Pokemon Lazarus Encounters PDF_

| Pokémon | Grass (Day) | Grass (Night) | Surfing | Old Rod | Good Rod | Super Rod | ☑ |
| --- | --- | --- | --- | --- | --- | --- | --- |
| <a href="../pokemon-lookup.html?q=aerodactyl">Aerodactyl</a> | 10% | 10% | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="corrin-crossing" data-species="aerodactyl" /> |
| <a href="../pokemon-lookup.html?q=braviary">Braviary</a> | 10% | — | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="corrin-crossing" data-species="braviary" /> |
| <a href="../pokemon-lookup.html?q=golduck">Golduck</a> | 10% | 10% | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="corrin-crossing" data-species="golduck" /> |
| <a href="../pokemon-lookup.html?q=kirlia">Kirlia</a> | 8% | 8% | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="corrin-crossing" data-species="kirlia" /> |
| <a href="../pokemon-lookup.html?q=magnemite">Magnemite</a> | 20% | 20% | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="corrin-crossing" data-species="magnemite" /> |
| <a href="../pokemon-lookup.html?q=torkoal">Torkoal</a> | 2% | 2% | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="corrin-crossing" data-species="torkoal" /> |
| <a href="../pokemon-lookup.html?q=victreebel">Victreebel</a> | 5% | — | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="corrin-crossing" data-species="victreebel" /> |
| <a href="../pokemon-lookup.html?q=weepinbell">Weepinbell</a> | 20% | — | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="corrin-crossing" data-species="weepinbell" /> |
| <a href="../pokemon-lookup.html?q=yanma">Yanma</a> | 10% | — | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="corrin-crossing" data-species="yanma" /> |
| <a href="../pokemon-lookup.html?q=yanmega">Yanmega</a> | 5% | — | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="corrin-crossing" data-species="yanmega" /> |
| <a href="../pokemon-lookup.html?q=farigiraf">Farigiraf</a> | — | 5% | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="corrin-crossing" data-species="farigiraf" /> |
| <a href="../pokemon-lookup.html?q=girafarig">Girafarig</a> | — | 10% | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="corrin-crossing" data-species="girafarig" /> |
| <a href="../pokemon-lookup.html?q=hisuian-braviary">Hisuian Braviary</a> | — | 10% | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="corrin-crossing" data-species="hisuian-braviary" /> |
| <a href="../pokemon-lookup.html?q=munna">Munna</a> | — | 20% | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="corrin-crossing" data-species="munna" /> |
| <a href="../pokemon-lookup.html?q=musharna">Musharna</a> | — | 5% | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="corrin-crossing" data-species="musharna" /> |
| <a href="../pokemon-lookup.html?q=bruxish">Bruxish</a> | — | — | 10% | — | — | 20% | <input type="checkbox" class="encounter-check" data-area="corrin-crossing" data-species="bruxish" /> |
| <a href="../pokemon-lookup.html?q=lumineon">Lumineon</a> | — | — | 30% | — | — | — | <input type="checkbox" class="encounter-check" data-area="corrin-crossing" data-species="lumineon" /> |
| <a href="../pokemon-lookup.html?q=tentacruel">Tentacruel</a> | — | — | 60% | — | — | — | <input type="checkbox" class="encounter-check" data-area="corrin-crossing" data-species="tentacruel" /> |
| <a href="../pokemon-lookup.html?q=magikarp">Magikarp</a> | — | — | — | 70% | — | — | <input type="checkbox" class="encounter-check" data-area="corrin-crossing" data-species="magikarp" /> |
| <a href="../pokemon-lookup.html?q=shellder">Shellder</a> | — | — | — | 30% | 20% | — | <input type="checkbox" class="encounter-check" data-area="corrin-crossing" data-species="shellder" /> |
| <a href="../pokemon-lookup.html?q=clamperl">Clamperl</a> | — | — | — | — | 20% | 40% | <input type="checkbox" class="encounter-check" data-area="corrin-crossing" data-species="clamperl" /> |
| <a href="../pokemon-lookup.html?q=psyduck">Psyduck</a> | — | — | — | — | 60% | — | <input type="checkbox" class="encounter-check" data-area="corrin-crossing" data-species="psyduck" /> |
| <a href="../pokemon-lookup.html?q=wailmer">Wailmer</a> | — | — | — | — | — | 40% | <input type="checkbox" class="encounter-check" data-area="corrin-crossing" data-species="wailmer" /> |

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

