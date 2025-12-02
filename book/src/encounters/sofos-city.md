<!-- area-id: sofos-city -->
### Sofos City

_Source: Pokemon Lazarus Encounters PDF_

| Pokémon | Grass (Day) | Grass (Night) | Surfing | Old Rod | Good Rod | Super Rod | ☑ |
| --- | --- | --- | --- | --- | --- | --- | --- |
| <a href="../pokemon-lookup.html?q=comfey">Comfey</a> | 10% | — | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="sofos-city" data-species="comfey" /> |
| <a href="../pokemon-lookup.html?q=corphish">Corphish</a> | 8% | 8% | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="sofos-city" data-species="corphish" /> |
| <a href="../pokemon-lookup.html?q=crabrawler">Crabrawler</a> | 10% | 10% | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="sofos-city" data-species="crabrawler" /> |
| <a href="../pokemon-lookup.html?q=girafarig">Girafarig</a> | 5% | — | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="sofos-city" data-species="girafarig" /> |
| <a href="../pokemon-lookup.html?q=helioptile">Helioptile</a> | 10% | — | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="sofos-city" data-species="helioptile" /> |
| <a href="../pokemon-lookup.html?q=honedge">Honedge</a> | 2% | 2% | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="sofos-city" data-species="honedge" /> |
| <a href="../pokemon-lookup.html?q=nosepass">Nosepass</a> | 5% | 5% | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="sofos-city" data-species="nosepass" /> |
| <a href="../pokemon-lookup.html?q=ponyta">Ponyta</a> | 10% | — | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="sofos-city" data-species="ponyta" /> |
| <a href="../pokemon-lookup.html?q=swablu">Swablu</a> | 20% | — | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="sofos-city" data-species="swablu" /> |
| <a href="../pokemon-lookup.html?q=wingull">Wingull</a> | 20% | 20% | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="sofos-city" data-species="wingull" /> |
| <a href="../pokemon-lookup.html?q=baltoy">Baltoy</a> | — | 10% | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="sofos-city" data-species="baltoy" /> |
| <a href="../pokemon-lookup.html?q=galarian-ponyta">Galarian Ponyta</a> | — | 10% | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="sofos-city" data-species="galarian-ponyta" /> |
| <a href="../pokemon-lookup.html?q=kirlia">Kirlia</a> | — | 10% | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="sofos-city" data-species="kirlia" /> |
| <a href="../pokemon-lookup.html?q=natu">Natu</a> | — | 5% | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="sofos-city" data-species="natu" /> |
| <a href="../pokemon-lookup.html?q=noctowl">Noctowl</a> | — | 20% | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="sofos-city" data-species="noctowl" /> |
| <a href="../pokemon-lookup.html?q=corsola">Corsola</a> | — | — | 5% | — | — | 15% | <input type="checkbox" class="encounter-check" data-area="sofos-city" data-species="corsola" /> |
| <a href="../pokemon-lookup.html?q=galarian-corsola">Galarian Corsola</a> | — | — | 5% | — | — | — | <input type="checkbox" class="encounter-check" data-area="sofos-city" data-species="galarian-corsola" /> |
| <a href="../pokemon-lookup.html?q=tentacool">Tentacool</a> | — | — | 60% | 30% | — | — | <input type="checkbox" class="encounter-check" data-area="sofos-city" data-species="tentacool" /> |
| <a href="../pokemon-lookup.html?q=wailmer">Wailmer</a> | — | — | 30% | — | — | 40% | <input type="checkbox" class="encounter-check" data-area="sofos-city" data-species="wailmer" /> |
| <a href="../pokemon-lookup.html?q=magikarp">Magikarp</a> | — | — | — | 70% | — | — | <input type="checkbox" class="encounter-check" data-area="sofos-city" data-species="magikarp" /> |
| <a href="../pokemon-lookup.html?q=clauncher">Clauncher</a> | — | — | — | — | 20% | 40% | <input type="checkbox" class="encounter-check" data-area="sofos-city" data-species="clauncher" /> |
| <a href="../pokemon-lookup.html?q=finneon">Finneon</a> | — | — | — | — | 20% | — | <input type="checkbox" class="encounter-check" data-area="sofos-city" data-species="finneon" /> |
| <a href="../pokemon-lookup.html?q=psyduck">Psyduck</a> | — | — | — | — | 60% | — | <input type="checkbox" class="encounter-check" data-area="sofos-city" data-species="psyduck" /> |

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

