<!-- area-id: port-pello -->
### Port Pello

_Source: Pokemon Lazarus Encounters PDF_

| Pokémon | Grass (Day) | Grass (Night) | Surfing | Old Rod | Good Rod | Super Rod | ☑ |
| --- | --- | --- | --- | --- | --- | --- | --- |
| <a href="../pokemon-lookup.html?q=alolan-raichu">Alolan Raichu</a> | 8% | — | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="port-pello" data-species="alolan-raichu" /> |
| <a href="../pokemon-lookup.html?q=cherrim">Cherrim</a> | 5% | — | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="port-pello" data-species="cherrim" /> |
| <a href="../pokemon-lookup.html?q=jigglypuff">Jigglypuff</a> | 20% | — | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="port-pello" data-species="jigglypuff" /> |
| <a href="../pokemon-lookup.html?q=linoone">Linoone</a> | 10% | — | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="port-pello" data-species="linoone" /> |
| <a href="../pokemon-lookup.html?q=pachirisu">Pachirisu</a> | 10% | — | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="port-pello" data-species="pachirisu" /> |
| <a href="../pokemon-lookup.html?q=rapidash">Rapidash</a> | 2% | — | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="port-pello" data-species="rapidash" /> |
| <a href="../pokemon-lookup.html?q=ribombee">Ribombee</a> | 10% | 10% | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="port-pello" data-species="ribombee" /> |
| <a href="../pokemon-lookup.html?q=togetic">Togetic</a> | 20% | — | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="port-pello" data-species="togetic" /> |
| <a href="../pokemon-lookup.html?q=tropius">Tropius</a> | 10% | 10% | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="port-pello" data-species="tropius" /> |
| <a href="../pokemon-lookup.html?q=wigglytuff">Wigglytuff</a> | 5% | — | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="port-pello" data-species="wigglytuff" /> |
| <a href="../pokemon-lookup.html?q=floette">Floette</a> | — | 20% | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="port-pello" data-species="floette" /> |
| <a href="../pokemon-lookup.html?q=florges">Florges</a> | — | 5% | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="port-pello" data-species="florges" /> |
| <a href="../pokemon-lookup.html?q=galarian-linoone">Galarian Linoone</a> | — | 10% | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="port-pello" data-species="galarian-linoone" /> |
| <a href="../pokemon-lookup.html?q=galarian-rapidash">Galarian Rapidash</a> | — | 2% | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="port-pello" data-species="galarian-rapidash" /> |
| <a href="../pokemon-lookup.html?q=grimmsnarl">Grimmsnarl</a> | — | 5% | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="port-pello" data-species="grimmsnarl" /> |
| <a href="../pokemon-lookup.html?q=mimikyu">Mimikyu</a> | — | 10% | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="port-pello" data-species="mimikyu" /> |
| <a href="../pokemon-lookup.html?q=morgrem">Morgrem</a> | — | 20% | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="port-pello" data-species="morgrem" /> |
| <a href="../pokemon-lookup.html?q=raichu">Raichu</a> | — | 8% | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="port-pello" data-species="raichu" /> |
| <a href="../pokemon-lookup.html?q=bruxish">Bruxish</a> | — | — | 10% | — | — | 20% | <input type="checkbox" class="encounter-check" data-area="port-pello" data-species="bruxish" /> |
| <a href="../pokemon-lookup.html?q=lumineon">Lumineon</a> | — | — | 30% | — | — | — | <input type="checkbox" class="encounter-check" data-area="port-pello" data-species="lumineon" /> |
| <a href="../pokemon-lookup.html?q=tentacruel">Tentacruel</a> | — | — | 60% | — | — | — | <input type="checkbox" class="encounter-check" data-area="port-pello" data-species="tentacruel" /> |
| <a href="../pokemon-lookup.html?q=magikarp">Magikarp</a> | — | — | — | 70% | — | — | <input type="checkbox" class="encounter-check" data-area="port-pello" data-species="magikarp" /> |
| <a href="../pokemon-lookup.html?q=shellder">Shellder</a> | — | — | — | 30% | 20% | — | <input type="checkbox" class="encounter-check" data-area="port-pello" data-species="shellder" /> |
| <a href="../pokemon-lookup.html?q=clamperl">Clamperl</a> | — | — | — | — | 20% | 40% | <input type="checkbox" class="encounter-check" data-area="port-pello" data-species="clamperl" /> |
| <a href="../pokemon-lookup.html?q=psyduck">Psyduck</a> | — | — | — | — | 60% | — | <input type="checkbox" class="encounter-check" data-area="port-pello" data-species="psyduck" /> |
| <a href="../pokemon-lookup.html?q=wailmer">Wailmer</a> | — | — | — | — | — | 40% | <input type="checkbox" class="encounter-check" data-area="port-pello" data-species="wailmer" /> |

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

