<!-- area-id: bronze-fields-north -->
### Bronze Fields (North)

_Source: Pokemon Lazarus Encounters PDF_

| Pokémon | Grass (Day) | Grass (Night) | ☑ |
| --- | --- | --- | --- |
| <a href="../pokemon-lookup.html?q=bellsprout">Bellsprout</a> | 5% | — | <input type="checkbox" class="encounter-check" data-area="bronze-fields-north" data-species="bellsprout" /> |
| <a href="../pokemon-lookup.html?q=cubone">Cubone</a> | 5% | 5% | <input type="checkbox" class="encounter-check" data-area="bronze-fields-north" data-species="cubone" /> |
| <a href="../pokemon-lookup.html?q=ducklett">Ducklett</a> | 5% | 5% | <input type="checkbox" class="encounter-check" data-area="bronze-fields-north" data-species="ducklett" /> |
| <a href="../pokemon-lookup.html?q=grubbin">Grubbin</a> | 10% | 10% | <input type="checkbox" class="encounter-check" data-area="bronze-fields-north" data-species="grubbin" /> |
| <a href="../pokemon-lookup.html?q=hoppip">Hoppip</a> | 10% | 10% | <input type="checkbox" class="encounter-check" data-area="bronze-fields-north" data-species="hoppip" /> |
| <a href="../pokemon-lookup.html?q=lillipup">Lillipup</a> | 20% | 20% | <input type="checkbox" class="encounter-check" data-area="bronze-fields-north" data-species="lillipup" /> |
| <a href="../pokemon-lookup.html?q=nincada">Nincada</a> | 20% | — | <input type="checkbox" class="encounter-check" data-area="bronze-fields-north" data-species="nincada" /> |
| <a href="../pokemon-lookup.html?q=phanpy">Phanpy</a> | 5% | 5% | <input type="checkbox" class="encounter-check" data-area="bronze-fields-north" data-species="phanpy" /> |
| <a href="../pokemon-lookup.html?q=pikipek">Pikipek</a> | 20% | — | <input type="checkbox" class="encounter-check" data-area="bronze-fields-north" data-species="pikipek" /> |
| <a href="../pokemon-lookup.html?q=hoothoot">Hoothoot</a> | — | 20% | <input type="checkbox" class="encounter-check" data-area="bronze-fields-north" data-species="hoothoot" /> |
| <a href="../pokemon-lookup.html?q=murkrow">Murkrow</a> | — | 5% | <input type="checkbox" class="encounter-check" data-area="bronze-fields-north" data-species="murkrow" /> |
| <a href="../pokemon-lookup.html?q=paras">Paras</a> | — | 20% | <input type="checkbox" class="encounter-check" data-area="bronze-fields-north" data-species="paras" /> |

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

