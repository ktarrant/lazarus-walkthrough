<!-- area-id: tower-of-dioxippus -->
### Tower of Dioxippus

_Source: Pokemon Lazarus Encounters PDF_

| Pokémon | Grass (Day) | Grass (Night) | ☑ |
| --- | --- | --- | --- |
| <a href="../pokemon-lookup.html?q=altaria">Altaria</a> | 5% | — | <input type="checkbox" class="encounter-check" data-area="tower-of-dioxippus" data-species="altaria" /> |
| <a href="../pokemon-lookup.html?q=appletun">Appletun</a> | 4% | 4% | <input type="checkbox" class="encounter-check" data-area="tower-of-dioxippus" data-species="appletun" /> |
| <a href="../pokemon-lookup.html?q=applin">Applin</a> | 20% | 20% | <input type="checkbox" class="encounter-check" data-area="tower-of-dioxippus" data-species="applin" /> |
| <a href="../pokemon-lookup.html?q=dipplin">Dipplin</a> | 2% | 2% | <input type="checkbox" class="encounter-check" data-area="tower-of-dioxippus" data-species="dipplin" /> |
| <a href="../pokemon-lookup.html?q=flapple">Flapple</a> | 4% | 4% | <input type="checkbox" class="encounter-check" data-area="tower-of-dioxippus" data-species="flapple" /> |
| <a href="../pokemon-lookup.html?q=gogoat">Gogoat</a> | 5% | 5% | <input type="checkbox" class="encounter-check" data-area="tower-of-dioxippus" data-species="gogoat" /> |
| <a href="../pokemon-lookup.html?q=goomy">Goomy</a> | 10% | 10% | <input type="checkbox" class="encounter-check" data-area="tower-of-dioxippus" data-species="goomy" /> |
| <a href="../pokemon-lookup.html?q=kecleon">Kecleon</a> | 10% | — | <input type="checkbox" class="encounter-check" data-area="tower-of-dioxippus" data-species="kecleon" /> |
| <a href="../pokemon-lookup.html?q=steenee">Steenee</a> | 10% | 10% | <input type="checkbox" class="encounter-check" data-area="tower-of-dioxippus" data-species="steenee" /> |
| <a href="../pokemon-lookup.html?q=swablu">Swablu</a> | 20% | — | <input type="checkbox" class="encounter-check" data-area="tower-of-dioxippus" data-species="swablu" /> |
| <a href="../pokemon-lookup.html?q=yanma">Yanma</a> | 10% | 10% | <input type="checkbox" class="encounter-check" data-area="tower-of-dioxippus" data-species="yanma" /> |
| <a href="../pokemon-lookup.html?q=drakloak">Drakloak</a> | — | 5% | <input type="checkbox" class="encounter-check" data-area="tower-of-dioxippus" data-species="drakloak" /> |
| <a href="../pokemon-lookup.html?q=dreepy">Dreepy</a> | — | 20% | <input type="checkbox" class="encounter-check" data-area="tower-of-dioxippus" data-species="dreepy" /> |
| <a href="../pokemon-lookup.html?q=mawile">Mawile</a> | — | 10% | <input type="checkbox" class="encounter-check" data-area="tower-of-dioxippus" data-species="mawile" /> |

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

