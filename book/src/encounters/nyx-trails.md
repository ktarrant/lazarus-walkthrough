<!-- area-id: nyx-trails -->
### Nyx Trails

_Source: Pokemon Lazarus Encounters PDF_

| Pokémon | Grass (Day) | Grass (Night) | Surfing | Old Rod | Good Rod | Super Rod | ☑ |
| --- | --- | --- | --- | --- | --- | --- | --- |
| <a href="../pokemon-lookup.html?q=abomasnow">Abomasnow</a> | 5% | 5% | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="nyx-trails" data-species="abomasnow" /> |
| <a href="../pokemon-lookup.html?q=banette">Banette</a> | 4% | 4% | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="nyx-trails" data-species="banette" /> |
| <a href="../pokemon-lookup.html?q=breloom">Breloom</a> | 10% | — | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="nyx-trails" data-species="breloom" /> |
| <a href="../pokemon-lookup.html?q=dusclops">Dusclops</a> | 4% | 4% | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="nyx-trails" data-species="dusclops" /> |
| <a href="../pokemon-lookup.html?q=gogoat">Gogoat</a> | 10% | — | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="nyx-trails" data-species="gogoat" /> |
| <a href="../pokemon-lookup.html?q=hisuian-electrode">Hisuian Electrode</a> | 2% | 2% | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="nyx-trails" data-species="hisuian-electrode" /> |
| <a href="../pokemon-lookup.html?q=lurantis">Lurantis</a> | 5% | — | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="nyx-trails" data-species="lurantis" /> |
| <a href="../pokemon-lookup.html?q=perrserker">Perrserker</a> | 10% | 10% | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="nyx-trails" data-species="perrserker" /> |
| <a href="../pokemon-lookup.html?q=persian">Persian</a> | 10% | — | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="nyx-trails" data-species="persian" /> |
| <a href="../pokemon-lookup.html?q=snover">Snover</a> | 20% | 20% | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="nyx-trails" data-species="snover" /> |
| <a href="../pokemon-lookup.html?q=xatu">Xatu</a> | 20% | — | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="nyx-trails" data-species="xatu" /> |
| <a href="../pokemon-lookup.html?q=alolan-persian">Alolan Persian</a> | — | 10% | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="nyx-trails" data-species="alolan-persian" /> |
| <a href="../pokemon-lookup.html?q=duskull">Duskull</a> | — | 10% | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="nyx-trails" data-species="duskull" /> |
| <a href="../pokemon-lookup.html?q=gourgeist">Gourgeist</a> | — | 5% | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="nyx-trails" data-species="gourgeist" /> |
| <a href="../pokemon-lookup.html?q=pumpkaboo">Pumpkaboo</a> | — | 20% | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="nyx-trails" data-species="pumpkaboo" /> |
| <a href="../pokemon-lookup.html?q=shuppet">Shuppet</a> | — | 10% | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="nyx-trails" data-species="shuppet" /> |
| <a href="../pokemon-lookup.html?q=bruxish">Bruxish</a> | — | — | 10% | — | — | 20% | <input type="checkbox" class="encounter-check" data-area="nyx-trails" data-species="bruxish" /> |
| <a href="../pokemon-lookup.html?q=lumineon">Lumineon</a> | — | — | 30% | — | — | — | <input type="checkbox" class="encounter-check" data-area="nyx-trails" data-species="lumineon" /> |
| <a href="../pokemon-lookup.html?q=tentacruel">Tentacruel</a> | — | — | 60% | — | — | — | <input type="checkbox" class="encounter-check" data-area="nyx-trails" data-species="tentacruel" /> |
| <a href="../pokemon-lookup.html?q=magikarp">Magikarp</a> | — | — | — | 70% | — | — | <input type="checkbox" class="encounter-check" data-area="nyx-trails" data-species="magikarp" /> |
| <a href="../pokemon-lookup.html?q=shellder">Shellder</a> | — | — | — | 30% | 20% | — | <input type="checkbox" class="encounter-check" data-area="nyx-trails" data-species="shellder" /> |
| <a href="../pokemon-lookup.html?q=clamperl">Clamperl</a> | — | — | — | — | 20% | 40% | <input type="checkbox" class="encounter-check" data-area="nyx-trails" data-species="clamperl" /> |
| <a href="../pokemon-lookup.html?q=psyduck">Psyduck</a> | — | — | — | — | 60% | — | <input type="checkbox" class="encounter-check" data-area="nyx-trails" data-species="psyduck" /> |
| <a href="../pokemon-lookup.html?q=wailmer">Wailmer</a> | — | — | — | — | — | 40% | <input type="checkbox" class="encounter-check" data-area="nyx-trails" data-species="wailmer" /> |

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

