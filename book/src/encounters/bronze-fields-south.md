<!-- area-id: bronze-fields-south -->
### Bronze Fields (South)

_Source: Pokemon Lazarus Encounters PDF_

| Pokémon | Grass (Day) | Grass (Night) | ☑ |
| --- | --- | --- | --- |
| <a href="../pokemon-lookup.html?q=comfey">Comfey</a> | 5% | — | <input type="checkbox" class="encounter-check" data-area="bronze-fields-south" data-species="comfey" /> |
| <a href="../pokemon-lookup.html?q=cubone">Cubone</a> | 2% | 2% | <input type="checkbox" class="encounter-check" data-area="bronze-fields-south" data-species="cubone" /> |
| <a href="../pokemon-lookup.html?q=cutiefly">Cutiefly</a> | 10% | — | <input type="checkbox" class="encounter-check" data-area="bronze-fields-south" data-species="cutiefly" /> |
| <a href="../pokemon-lookup.html?q=fomantis">Fomantis</a> | 10% | — | <input type="checkbox" class="encounter-check" data-area="bronze-fields-south" data-species="fomantis" /> |
| <a href="../pokemon-lookup.html?q=grubbin">Grubbin</a> | 4% | 4% | <input type="checkbox" class="encounter-check" data-area="bronze-fields-south" data-species="grubbin" /> |
| <a href="../pokemon-lookup.html?q=hoppip">Hoppip</a> | 10% | — | <input type="checkbox" class="encounter-check" data-area="bronze-fields-south" data-species="hoppip" /> |
| <a href="../pokemon-lookup.html?q=lillipup">Lillipup</a> | 20% | 20% | <input type="checkbox" class="encounter-check" data-area="bronze-fields-south" data-species="lillipup" /> |
| <a href="../pokemon-lookup.html?q=mareep">Mareep</a> | 20% | 20% | <input type="checkbox" class="encounter-check" data-area="bronze-fields-south" data-species="mareep" /> |
| <a href="../pokemon-lookup.html?q=mudbray">Mudbray</a> | 9% | 9% | <input type="checkbox" class="encounter-check" data-area="bronze-fields-south" data-species="mudbray" /> |
| <a href="../pokemon-lookup.html?q=nincada">Nincada</a> | 10% | 10% | <input type="checkbox" class="encounter-check" data-area="bronze-fields-south" data-species="nincada" /> |
| <a href="../pokemon-lookup.html?q=murkrow">Murkrow</a> | — | 20% | <input type="checkbox" class="encounter-check" data-area="bronze-fields-south" data-species="murkrow" /> |
| <a href="../pokemon-lookup.html?q=vullaby">Vullaby</a> | — | 10% | <input type="checkbox" class="encounter-check" data-area="bronze-fields-south" data-species="vullaby" /> |
| <a href="../pokemon-lookup.html?q=yamper">Yamper</a> | — | 5% | <input type="checkbox" class="encounter-check" data-area="bronze-fields-south" data-species="yamper" /> |

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

