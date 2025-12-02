<!-- area-id: asfal-hills -->
### Asfal Hills

_Source: Pokemon Lazarus Encounters PDF_

| Pokémon | Grass (Day) | Grass (Night) | ☑ |
| --- | --- | --- | --- |
| <a href="../pokemon-lookup.html?q=buizel">Buizel</a> | 5% | — | <input type="checkbox" class="encounter-check" data-area="asfal-hills" data-species="buizel" /> |
| <a href="../pokemon-lookup.html?q=chatot">Chatot</a> | 6% | — | <input type="checkbox" class="encounter-check" data-area="asfal-hills" data-species="chatot" /> |
| <a href="../pokemon-lookup.html?q=galarian-meowth">Galarian Meowth</a> | 10% | 10% | <input type="checkbox" class="encounter-check" data-area="asfal-hills" data-species="galarian-meowth" /> |
| <a href="../pokemon-lookup.html?q=hawlucha">Hawlucha</a> | 4% | 4% | <input type="checkbox" class="encounter-check" data-area="asfal-hills" data-species="hawlucha" /> |
| <a href="../pokemon-lookup.html?q=pachirisu">Pachirisu</a> | 10% | — | <input type="checkbox" class="encounter-check" data-area="asfal-hills" data-species="pachirisu" /> |
| <a href="../pokemon-lookup.html?q=pawmi">Pawmi</a> | 15% | 15% | <input type="checkbox" class="encounter-check" data-area="asfal-hills" data-species="pawmi" /> |
| <a href="../pokemon-lookup.html?q=tauros">Tauros</a> | 20% | — | <input type="checkbox" class="encounter-check" data-area="asfal-hills" data-species="tauros" /> |
| <a href="../pokemon-lookup.html?q=tauros-combat-breed">Tauros Combat Breed</a> | 20% | — | <input type="checkbox" class="encounter-check" data-area="asfal-hills" data-species="tauros-combat-breed" /> |
| <a href="../pokemon-lookup.html?q=yamper">Yamper</a> | 10% | 10% | <input type="checkbox" class="encounter-check" data-area="asfal-hills" data-species="yamper" /> |
| <a href="../pokemon-lookup.html?q=dedenne">Dedenne</a> | — | 10% | <input type="checkbox" class="encounter-check" data-area="asfal-hills" data-species="dedenne" /> |
| <a href="../pokemon-lookup.html?q=shroodle">Shroodle</a> | — | 5% | <input type="checkbox" class="encounter-check" data-area="asfal-hills" data-species="shroodle" /> |
| <a href="../pokemon-lookup.html?q=stunky">Stunky</a> | — | 6% | <input type="checkbox" class="encounter-check" data-area="asfal-hills" data-species="stunky" /> |
| <a href="../pokemon-lookup.html?q=tauros-aqua-breed">Tauros Aqua Breed</a> | — | 20% | <input type="checkbox" class="encounter-check" data-area="asfal-hills" data-species="tauros-aqua-breed" /> |
| <a href="../pokemon-lookup.html?q=tauros-blaze-breed">Tauros Blaze Breed</a> | — | 20% | <input type="checkbox" class="encounter-check" data-area="asfal-hills" data-species="tauros-blaze-breed" /> |

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

