<!-- area-id: kaptara-island-west -->
### Kaptara Island (West)

_Source: Pokemon Lazarus Encounters PDF_

| Pokémon | Grass (Day) | Grass (Night) | ☑ |
| --- | --- | --- | --- |
| <a href="../pokemon-lookup.html?q=altaria">Altaria</a> | 5% | 5% | <input type="checkbox" class="encounter-check" data-area="kaptara-island-west" data-species="altaria" /> |
| <a href="../pokemon-lookup.html?q=duosion">Duosion</a> | 8% | — | <input type="checkbox" class="encounter-check" data-area="kaptara-island-west" data-species="duosion" /> |
| <a href="../pokemon-lookup.html?q=fomantis">Fomantis</a> | 20% | 20% | <input type="checkbox" class="encounter-check" data-area="kaptara-island-west" data-species="fomantis" /> |
| <a href="../pokemon-lookup.html?q=lurantis">Lurantis</a> | 5% | 5% | <input type="checkbox" class="encounter-check" data-area="kaptara-island-west" data-species="lurantis" /> |
| <a href="../pokemon-lookup.html?q=morgrem">Morgrem</a> | 10% | 10% | <input type="checkbox" class="encounter-check" data-area="kaptara-island-west" data-species="morgrem" /> |
| <a href="../pokemon-lookup.html?q=oranguru">Oranguru</a> | 10% | — | <input type="checkbox" class="encounter-check" data-area="kaptara-island-west" data-species="oranguru" /> |
| <a href="../pokemon-lookup.html?q=passimian">Passimian</a> | 10% | — | <input type="checkbox" class="encounter-check" data-area="kaptara-island-west" data-species="passimian" /> |
| <a href="../pokemon-lookup.html?q=reuniclus">Reuniclus</a> | 2% | — | <input type="checkbox" class="encounter-check" data-area="kaptara-island-west" data-species="reuniclus" /> |
| <a href="../pokemon-lookup.html?q=trumbeak">Trumbeak</a> | 20% | 20% | <input type="checkbox" class="encounter-check" data-area="kaptara-island-west" data-species="trumbeak" /> |
| <a href="../pokemon-lookup.html?q=yanma">Yanma</a> | 10% | — | <input type="checkbox" class="encounter-check" data-area="kaptara-island-west" data-species="yanma" /> |
| <a href="../pokemon-lookup.html?q=claydol">Claydol</a> | — | 10% | <input type="checkbox" class="encounter-check" data-area="kaptara-island-west" data-species="claydol" /> |
| <a href="../pokemon-lookup.html?q=espathra">Espathra</a> | — | 10% | <input type="checkbox" class="encounter-check" data-area="kaptara-island-west" data-species="espathra" /> |
| <a href="../pokemon-lookup.html?q=golduck">Golduck</a> | — | 10% | <input type="checkbox" class="encounter-check" data-area="kaptara-island-west" data-species="golduck" /> |
| <a href="../pokemon-lookup.html?q=gothitelle">Gothitelle</a> | — | 2% | <input type="checkbox" class="encounter-check" data-area="kaptara-island-west" data-species="gothitelle" /> |
| <a href="../pokemon-lookup.html?q=gothorita">Gothorita</a> | — | 8% | <input type="checkbox" class="encounter-check" data-area="kaptara-island-west" data-species="gothorita" /> |

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

