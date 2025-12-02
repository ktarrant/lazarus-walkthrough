<!-- area-id: areios-hideout -->
### Areios Hideout

_Source: Pokemon Lazarus Encounters PDF_

| Pokémon | Grass (Day) | Grass (Night) | ☑ |
| --- | --- | --- | --- |
| <a href="../pokemon-lookup.html?q=ariados">Ariados</a> | 20% | — | <input type="checkbox" class="encounter-check" data-area="areios-hideout" data-species="ariados" /> |
| <a href="../pokemon-lookup.html?q=boltund">Boltund</a> | 10% | 10% | <input type="checkbox" class="encounter-check" data-area="areios-hideout" data-species="boltund" /> |
| <a href="../pokemon-lookup.html?q=houndoom">Houndoom</a> | 20% | 20% | <input type="checkbox" class="encounter-check" data-area="areios-hideout" data-species="houndoom" /> |
| <a href="../pokemon-lookup.html?q=lurantis">Lurantis</a> | 8% | — | <input type="checkbox" class="encounter-check" data-area="areios-hideout" data-species="lurantis" /> |
| <a href="../pokemon-lookup.html?q=muk">Muk</a> | 10% | — | <input type="checkbox" class="encounter-check" data-area="areios-hideout" data-species="muk" /> |
| <a href="../pokemon-lookup.html?q=obstagoon">Obstagoon</a> | 10% | 10% | <input type="checkbox" class="encounter-check" data-area="areios-hideout" data-species="obstagoon" /> |
| <a href="../pokemon-lookup.html?q=primeape">Primeape</a> | 10% | 10% | <input type="checkbox" class="encounter-check" data-area="areios-hideout" data-species="primeape" /> |
| <a href="../pokemon-lookup.html?q=salandit">Salandit</a> | 10% | 10% | <input type="checkbox" class="encounter-check" data-area="areios-hideout" data-species="salandit" /> |
| <a href="../pokemon-lookup.html?q=tyrunt">Tyrunt</a> | 2% | 2% | <input type="checkbox" class="encounter-check" data-area="areios-hideout" data-species="tyrunt" /> |
| <a href="../pokemon-lookup.html?q=alolan-muk">Alolan Muk</a> | — | 10% | <input type="checkbox" class="encounter-check" data-area="areios-hideout" data-species="alolan-muk" /> |
| <a href="../pokemon-lookup.html?q=drapion">Drapion</a> | — | 20% | <input type="checkbox" class="encounter-check" data-area="areios-hideout" data-species="drapion" /> |
| <a href="../pokemon-lookup.html?q=scrafty">Scrafty</a> | — | 8% | <input type="checkbox" class="encounter-check" data-area="areios-hideout" data-species="scrafty" /> |

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

